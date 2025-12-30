// Generated macro for impl_180 (impl)
macro_rules! Depcrate_helpers_metricsimpl_180 {
() => {
// Module: crate::helpers::metrics
// Provides: {"impl_180"}
// Dependencies: {}
impl MetricMap { pub fn new () -> MetricMap { MetricMap (BTreeMap :: new ()) } # [doc = " Insert a named `value` (+/- `noise`) metric into the map. The value"] # [doc = " must be non-negative. The `noise` indicates the uncertainty of the"] # [doc = " metric, which doubles as the \"noise range\" of acceptable"] # [doc = " pairwise-regressions on this named value, when comparing from one"] # [doc = " metric to the next using `compare_to_old`."] # [doc = ""] # [doc = " If `noise` is positive, then it means this metric is of a value"] # [doc = " you want to see grow smaller, so a change larger than `noise` in the"] # [doc = " positive direction represents a regression."] # [doc = ""] # [doc = " If `noise` is negative, then it means this metric is of a value"] # [doc = " you want to see grow larger, so a change larger than `noise` in the"] # [doc = " negative direction represents a regression."] pub fn insert_metric (& mut self , name : & str , value : f64 , noise : f64) { let m = Metric { value , noise } ; self . 0 . insert (name . to_owned () , m) ; } pub fn fmt_metrics (& self) -> String { let v = self . 0 . iter () . map (| (k , v) | format ! ("{}: {} (+/- {})" , * k , v . value , v . noise)) . collect :: < Vec < _ > > () ; v . join (", ") } }
};
}
