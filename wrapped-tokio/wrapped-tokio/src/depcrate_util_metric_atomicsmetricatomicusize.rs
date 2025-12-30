// Generated macro for MetricAtomicUsize (struct)
macro_rules! Depcrate_util_metric_atomicsMetricAtomicUsize {
() => {
// Module: crate::util::metric_atomics
// Provides: {"MetricAtomicUsize"}
// Dependencies: {}
# [cfg_attr (not (all (tokio_unstable , feature = "rt")) , allow (dead_code))] # [doc = " `AtomicUsize` for use in metrics."] # [doc = ""] # [doc = " This exposes simplified APIs for use in metrics & uses `std::sync` instead of Loom to avoid polluting loom logs with metric information."] # [derive (Debug , Default)] pub (crate) struct MetricAtomicUsize { value : AtomicUsize , }
};
}
