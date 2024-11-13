// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;
use glean_core::Lifetime;

#[test]
fn pings_with_follows_false_are_exempt() {
    let (mut glean, _t) = new_glean(None);

    let ping_name = "nofollows";
    // When `follows_collection_enabled=false` then by default `enabled=false`
    let ping = PingType::new(
        ping_name,
        /* include_client_id */ false,
        /* send_if_empty */ true,
        /* precise_timestamps */ true,
        /* include_info_sections */ false,
        /* enabled */ false,
        vec![],
        vec![],
    )
    .with_follows(false);
    glean.register_ping_type(&ping);

    // We need to store a metric as an empty ping is not stored.
    let counter = CounterMetric::new(CommonMetricData {
        name: "counter".into(),
        category: "local".into(),
        send_in_pings: vec!["nofollows".into()],
        ..Default::default()
    });

    counter.add_sync(&glean, 1);
    assert!(!ping.submit_sync(&glean, None));

    glean.set_ping_enabled(ping_name.to_string(), true);
    counter.add_sync(&glean, 2);
    assert!(ping.submit_sync(&glean, None));

    let mut queued_pings = get_queued_pings(glean.get_data_path()).unwrap();
    assert_eq!(1, queued_pings.len());

    let json = queued_pings.pop().unwrap().1;
    let counter_val = json["metrics"]["counter"]["local.counter"]
        .as_i64()
        .unwrap();
    assert_eq!(2, counter_val);

    glean.set_upload_enabled(false);
    assert_eq!(1, get_queued_pings(glean.get_data_path()).unwrap().len());
    // Disabling upload generates a deletion ping
    assert_eq!(1, get_deletion_pings(glean.get_data_path()).unwrap().len());

    // Regardless, the `nofollows` ping is still enabled.
    counter.add_sync(&glean, 10);
    assert!(ping.submit_sync(&glean, None));

    let queued_pings = get_queued_pings(glean.get_data_path()).unwrap();
    // both `nofollows` pings remain in the queue
    assert_eq!(2, queued_pings.len());

    let mut values = vec![2, 10];
    for ping in queued_pings {
        let json = ping.1;
        let counter_val = json["metrics"]["counter"]["local.counter"]
            .as_i64()
            .unwrap();

        assert!(values.contains(&counter_val));
        values.retain(|&x| x != counter_val);
    }
}

#[test]
fn nofollows_ping_can_ride_along() {
    let (mut glean, _t) = new_glean(None);

    // When `follows_collection_enabled=false` then by default `enabled=false`
    let nofollows_ping = PingType::new(
        "nofollows",
        /* include_client_id */ false,
        /* send_if_empty */ true,
        /* precise_timestamps */ true,
        /* include_info_sections */ false,
        /* enabled */ false,
        vec![],
        vec![],
    )
    .with_follows(false);
    glean.register_ping_type(&nofollows_ping);

    let manual_ping = PingType::new(
        "manual",
        /* include_client_id */ true,
        /* send_if_empty */ false,
        /* precise_timestamps */ true,
        /* include_info_sections */ true,
        /* enabled */ true,
        vec!["nofollows".to_string()],
        vec!["triggered".to_string()],
    );
    glean.register_ping_type(&manual_ping);

    // We need to store a metric as an empty ping is not stored.
    let counter = CounterMetric::new(CommonMetricData {
        name: "counter".into(),
        category: "local".into(),
        send_in_pings: vec!["manual".into(), "nofollows".into()],
        lifetime: Lifetime::Application,
        ..Default::default()
    });

    // Trigger a ping with data.
    counter.add_sync(&glean, 1);
    assert!(manual_ping.submit_sync(&glean, Some("triggered")));
    assert_eq!(1, get_queued_pings(glean.get_data_path()).unwrap().len());

    // Enable one more ping, trigger it with more data
    glean.set_ping_enabled("nofollows".to_string(), true);
    counter.add_sync(&glean, 2);
    assert!(manual_ping.submit_sync(&glean, Some("triggered")));
    // The previous one, plus 2 new ones
    let queued_pings = get_queued_pings(glean.get_data_path()).unwrap();
    assert_eq!(3, queued_pings.len());
    for (_url, json, info) in queued_pings {
        let Some(obj) = info else {
            panic!("no ping info")
        };
        let counter_val = json["metrics"]["counter"]["local.counter"]
            .as_i64()
            .unwrap();

        if obj["ping_name"].as_str().unwrap() == "nofollows" {
            assert_eq!(2, counter_val, "{:?}", json);
        } else {
            let seq = json["ping_info"]["seq"].as_i64().unwrap();

            match seq {
                0 => assert_eq!(1, counter_val),
                1 => assert_eq!(3, counter_val),
                2 => assert_eq!(8, counter_val),
                _ => panic!("unexpected sequence number: {}", seq),
            }
        }
    }

    // Disable it again
    glean.set_ping_enabled("nofollows".to_string(), false);
    counter.add_sync(&glean, 5);
    assert!(manual_ping.submit_sync(&glean, Some("triggered")));
    let queued_pings = get_queued_pings(glean.get_data_path()).unwrap();
    // The 2 previous `manual` pings, the `nofollows` was removed, plus the new `manual` one
    assert_eq!(3, queued_pings.len());

    // Check that all follows are as expected.
    // We cannot guarantee order, so we need to look at some values
    for (_url, json, info) in queued_pings {
        let Some(obj) = info else {
            panic!("no ping info")
        };
        let counter_val = json["metrics"]["counter"]["local.counter"]
            .as_i64()
            .unwrap();

        assert_ne!("nofollows", obj["ping_name"].as_str().unwrap());
        let seq = json["ping_info"]["seq"].as_i64().unwrap();

        match seq {
            0 => assert_eq!(1, counter_val),
            1 => assert_eq!(3, counter_val),
            2 => assert_eq!(8, counter_val),
            _ => panic!("unexpected sequence number: {}", seq),
        }
    }
}

#[test]
fn queued_nofollows_pings_are_not_removed() {
    let (mut glean, _t) = new_glean(None);

    // When `follows_collection_enabled=false` then by default `enabled=false`
    let nofollows_ping = PingType::new(
        "nofollows",
        /* include_client_id */ false,
        /* send_if_empty */ true,
        /* precise_timestamps */ true,
        /* include_info_sections */ false,
        /* enabled */ false,
        vec![],
        vec![],
    )
    .with_follows(false);
    glean.register_ping_type(&nofollows_ping);
    glean.set_ping_enabled("nofollows".to_string(), true);

    let manual_ping = PingType::new(
        "manual",
        /* include_client_id */ true,
        /* send_if_empty */ false,
        /* precise_timestamps */ true,
        /* include_info_sections */ true,
        /* enabled */ true,
        vec![],
        vec![],
    );
    glean.register_ping_type(&manual_ping);

    // We need to store a metric as an empty ping is not stored.
    let counter = CounterMetric::new(CommonMetricData {
        name: "counter".into(),
        category: "local".into(),
        send_in_pings: vec!["manual".into(), "nofollows".into()],
        lifetime: Lifetime::Application,
        ..Default::default()
    });

    // Trigger a ping with data.
    counter.add_sync(&glean, 1);
    assert!(manual_ping.submit_sync(&glean, Some("triggered")));
    assert!(nofollows_ping.submit_sync(&glean, Some("triggered")));
    assert_eq!(2, get_queued_pings(glean.get_data_path()).unwrap().len());

    glean.set_upload_enabled(false);
    assert_eq!(1, get_queued_pings(glean.get_data_path()).unwrap().len());
}
