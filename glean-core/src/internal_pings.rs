// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use malloc_size_of_derive::MallocSizeOf;

use crate::metrics::PingType;

/// Glean-provided pings, all enabled by default.
///
/// These pings are defined in `glean-core/pings.yaml` and for now manually translated into Rust code.
/// This might get auto-generated when the Rust API lands ([Bug 1579146](https://bugzilla.mozilla.org/show_bug.cgi?id=1579146)).
///
/// They are parsed and registered by the platform-specific wrappers, but might be used Glean-internal directly.
#[derive(Debug, Clone, MallocSizeOf)]
pub struct InternalPings {
    pub baseline: PingType,
    pub metrics: PingType,
    pub events: PingType,
    pub deletion_request: PingType,
}

impl InternalPings {
    pub fn new(enabled: bool) -> InternalPings {
        InternalPings {
            baseline: PingType::new_internal(
                "baseline",
                true,
                true,
                true,
                true,
                vec![
                    "active".to_string(),
                    "dirty_startup".to_string(),
                    "inactive".to_string(),
                ],
                enabled,
            ),
            metrics: PingType::new_internal(
                "metrics",
                true,
                false,
                true,
                true,
                vec![
                    "overdue".to_string(),
                    "reschedule".to_string(),
                    "today".to_string(),
                    "tomorrow".to_string(),
                    "upgrade".to_string(),
                ],
                enabled,
            ),
            events: PingType::new_internal(
                "events",
                true,
                false,
                true,
                true,
                vec![
                    "startup".to_string(),
                    "inactive".to_string(),
                    "max_capacity".to_string(),
                ],
                enabled,
            ),
            deletion_request: PingType::new(
                "deletion-request",
                true,
                true,
                true,
                true,
                vec!["at_init".to_string(), "set_upload_enabled".to_string()],
            ),
        }
    }
}
