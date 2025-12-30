// Generated macro for fmt_bench_samples (function)
macro_rules! Depcrate_benchfmt_bench_samples {
() => {
// Module: crate::bench
// Provides: {"fmt_bench_samples"}
// Dependencies: {}
pub fn fmt_bench_samples (bs : & BenchSamples) -> String { use std :: fmt :: Write ; let mut output = String :: new () ; let median = bs . ns_iter_summ . median ; let deviation = bs . ns_iter_summ . max - bs . ns_iter_summ . min ; write ! (output , "{:>14} ns/iter (+/- {})" , fmt_thousands_sep (median , ',') , fmt_thousands_sep (deviation , ',')) . unwrap () ; if bs . mb_s != 0 { write ! (output , " = {} MB/s" , bs . mb_s) . unwrap () ; } output }
};
}
