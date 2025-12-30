// Generated macro for impl_244 (impl)
macro_rules! Depcrate_statsimpl_244 {
() => {
// Module: crate::stats
// Provides: {"impl_244"}
// Dependencies: {}
impl Summary { # [doc = " Constructs a new summary of a sample set."] pub fn new (samples : & [f64]) -> Summary { Summary { sum : samples . sum () , min : samples . min () , max : samples . max () , mean : samples . mean () , median : samples . median () , var : samples . var () , std_dev : samples . std_dev () , std_dev_pct : samples . std_dev_pct () , median_abs_dev : samples . median_abs_dev () , median_abs_dev_pct : samples . median_abs_dev_pct () , quartiles : samples . quartiles () , iqr : samples . iqr () , } } }
};
}
