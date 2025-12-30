// Generated macro for winsorize (function)
macro_rules! Depcrate_statswinsorize {
() => {
// Module: crate::stats
// Provides: {"winsorize"}
// Dependencies: {}
# [doc = " Winsorize a set of samples, replacing values above the `100-pct` percentile"] # [doc = " and below the `pct` percentile with those percentiles themselves. This is a"] # [doc = " way of minimizing the effect of outliers, at the cost of biasing the sample."] # [doc = " It differs from trimming in that it does not change the number of samples,"] # [doc = " just changes the values of those that are outliers."] # [doc = ""] # [doc = " See: <https://en.wikipedia.org/wiki/Winsorising>"] pub fn winsorize (samples : & mut [f64] , pct : f64) { let mut tmp = samples . to_vec () ; local_sort (& mut tmp) ; let lo = percentile_of_sorted (& tmp , pct) ; let hundred = 100_f64 ; let hi = percentile_of_sorted (& tmp , hundred - pct) ; for samp in samples { if * samp > hi { * samp = hi } else if * samp < lo { * samp = lo } } }
};
}
