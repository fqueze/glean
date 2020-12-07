(function() {var implementors = {};
implementors["glean"] = [{"text":"impl !UnwindSafe for Configuration","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ClientInfoMetrics","synthetic":true,"types":[]},{"text":"impl UnwindSafe for HttpUploader","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UuidMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BooleanMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for CounterMetric","synthetic":true,"types":[]},{"text":"impl&lt;K&gt; UnwindSafe for EventMetric&lt;K&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;K: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for LabeledMetric&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;T as Sealed&gt;::Inner: RefUnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PingType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for QuantityMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecordedExperimentData","synthetic":true,"types":[]},{"text":"impl UnwindSafe for StringMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TimespanMetric","synthetic":true,"types":[]}];
implementors["glean_core"] = [{"text":"impl UnwindSafe for CommonMetricData","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for Error","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Configuration","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Glean","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Lifetime","synthetic":true,"types":[]},{"text":"impl !UnwindSafe for ErrorKind","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ErrorType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for RecordedEvent","synthetic":true,"types":[]},{"text":"impl UnwindSafe for HistogramType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for BooleanMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for CounterMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for CustomDistributionMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DatetimeMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EventMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for JweMetric","synthetic":true,"types":[]},{"text":"impl&lt;T&gt; UnwindSafe for LabeledMetric&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: UnwindSafe,&nbsp;</span>","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MemoryDistributionMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PingType","synthetic":true,"types":[]},{"text":"impl UnwindSafe for QuantityMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for StringMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for StringListMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TimespanMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TimingDistributionMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UuidMetric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for DistributionData","synthetic":true,"types":[]},{"text":"impl UnwindSafe for MemoryUnit","synthetic":true,"types":[]},{"text":"impl UnwindSafe for TimeUnit","synthetic":true,"types":[]},{"text":"impl UnwindSafe for Metric","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PingMaker","synthetic":true,"types":[]},{"text":"impl UnwindSafe for StorageManager","synthetic":true,"types":[]},{"text":"impl UnwindSafe for EventRecordingError","synthetic":true,"types":[]},{"text":"impl UnwindSafe for NoExtraKeys","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PingRequest","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PingUploadManager","synthetic":true,"types":[]},{"text":"impl UnwindSafe for UploadResult","synthetic":true,"types":[]},{"text":"impl UnwindSafe for PingUploadTask","synthetic":true,"types":[]}];
implementors["glean_ffi"] = [{"text":"impl&lt;'a&gt; UnwindSafe for FfiConfiguration&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl UnwindSafe for ByteBuffer","synthetic":true,"types":[]},{"text":"impl UnwindSafe for FfiPingUploadTask","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()