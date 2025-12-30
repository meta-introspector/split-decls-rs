// Generated macro for MetricAtomicU64 (struct)
macro_rules! Depcrate_util_metric_atomicsMetricAtomicU64 {
() => {
// Module: crate::util::metric_atomics
// Provides: {"MetricAtomicU64"}
// Dependencies: {}
# [doc = " `AtomicU64` that is a no-op on platforms without 64-bit atomics"] # [doc = ""] # [doc = " When used on platforms without 64-bit atomics, writes to this are no-ops."] # [doc = " The `load` method is only defined when 64-bit atomics are available."] # [derive (Debug , Default)] pub (crate) struct MetricAtomicU64 { # [cfg (target_has_atomic = "64")] value : AtomicU64 , }
};
}
