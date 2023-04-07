window.SIDEBAR_ITEMS = {"enum":[["ErrorType","The possible error types for metric recording. Note: the cases in this enum must be kept in sync with the ones in the platform-specific code (e.g. `ErrorType.kt`) and with the metrics in the registry files."],["HistogramType","Different kinds of histograms."],["Lifetime","The supported metrics’ lifetimes."],["MemoryUnit","Different resolutions supported by the memory related metric types (e.g. MemoryDistributionMetric)."],["TimeUnit","Different resolutions supported by the time related metric types (e.g. DatetimeMetric)."]],"fn":[["get_timestamp_ms","Returns a timestamp corresponding to “now” with millisecond precision."],["glean_set_metrics_enabled_config","Set the remote configuration values for the metrics’ disabled property"],["handle_client_active","Performs the collection/cleanup operations required by becoming active."],["handle_client_inactive","Performs the collection/cleanup operations required by becoming inactive."],["initialize","Creates and initializes a new Glean object."],["persist_ping_lifetime_data","Asks the database to persist ping-lifetime data to disk. Probably expensive to call. Only has effect when Glean is configured with `delay_ping_lifetime_io: true`. If Glean hasn’t been initialized this will dispatch and return Ok(()), otherwise it will block until the persist is done and return its Result."],["set_debug_view_tag","Sets a debug view tag."],["set_experiment_active","Indicate that an experiment is running.  Glean will then add an experiment annotation to the environment which is sent with pings. This infomration is not persisted between runs."],["set_experiment_inactive","Indicate that an experiment is no longer running."],["set_log_pings","Sets the log pings debug option."],["set_source_tags","Sets source tags."],["set_upload_enabled","Sets whether upload is enabled or not."],["shutdown","Shuts down Glean in an orderly fashion."],["submit_ping_by_name","Collects and submits a ping for eventual uploading by name."],["test_get_experiment_data","TEST ONLY FUNCTION. Returns the [`RecordedExperiment`] for the given `experiment_id` or panics if the id isn’t found."],["test_is_experiment_active","TEST ONLY FUNCTION. Checks if an experiment is currently active."],["test_reset_glean","TEST ONLY FUNCTION. Resets the Glean state and triggers init again."]],"mod":[["net","Handling the Glean upload logic."],["private","The different metric types supported by the Glean SDK to handle data."],["traits","API definitions for the different metric types supported by the Glean SDK."]],"struct":[["ClientInfoMetrics","Metrics included in every ping as `client_info`."],["CommonMetricData","The common set of data shared across all different metric types."],["Configuration","The Glean configuration."],["ConfigurationBuilder","Configuration builder."],["Datetime","Representation of a date, time and timezone."],["DistributionData","A snapshot of all buckets and the accumulated sum of a distribution."],["Error","A specialized `Error` type for this crate’s operations."],["Glean","The object holding meta information about a Glean instance."],["Rate","A rate value as given by its numerator and denominator."],["RecordedEvent","Represents the recorded data for a single event."],["RecordedExperiment","Deserialized experiment data."],["TimerId","Identifier for a running timer."]],"type":[["Result","A specialized `Result` type for this crate’s operations."]]};