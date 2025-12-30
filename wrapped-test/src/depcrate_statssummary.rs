// Generated macro for Summary (struct)
macro_rules! Depcrate_statsSummary {
() => {
// Module: crate::stats
// Provides: {"Summary"}
// Dependencies: {}
# [doc = " Extracted collection of all the summary statistics of a sample set."] # [derive (Debug , Clone , PartialEq , Copy)] # [allow (missing_docs)] pub struct Summary { pub sum : f64 , pub min : f64 , pub max : f64 , pub mean : f64 , pub median : f64 , pub var : f64 , pub std_dev : f64 , pub std_dev_pct : f64 , pub median_abs_dev : f64 , pub median_abs_dev_pct : f64 , pub quartiles : (f64 , f64 , f64) , pub iqr : f64 , }
};
}
