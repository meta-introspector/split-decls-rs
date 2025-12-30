// Generated macro for parse_metrics (function)
macro_rules! Depcrate_metricsparse_metrics {
() => {
// Module: crate::metrics
// Provides: {"parse_metrics"}
// Dependencies: {}
fn parse_metrics (output : & str) -> Vec < (& str , u64 , & str) > { output . lines () . filter_map (| it | { let entry = it . split (':') . collect :: < Vec < _ > > () ; match entry . as_slice () { ["METRIC" , name , value , unit] => Some ((* name , value . parse () . unwrap () , * unit)) , _ => None , } }) . collect () }
};
}
