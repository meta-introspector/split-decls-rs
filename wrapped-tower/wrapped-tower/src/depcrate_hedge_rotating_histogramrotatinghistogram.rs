// Generated macro for RotatingHistogram (struct)
macro_rules! Depcrate_hedge_rotating_histogramRotatingHistogram {
() => {
// Module: crate::hedge::rotating_histogram
// Provides: {"RotatingHistogram"}
// Dependencies: {}
# [doc = " This represents a \"rotating\" histogram which stores two histogram, one which"] # [doc = " should be read and one which should be written to.  Every period, the read"] # [doc = " histogram is discarded and replaced by the write histogram.  The idea here"] # [doc = " is that the read histogram should always contain a full period (the previous"] # [doc = " period) of write operations."] # [derive (Debug)] pub struct RotatingHistogram { read : Histogram < u64 > , write : Histogram < u64 > , last_rotation : Instant , period : Duration , }
};
}
