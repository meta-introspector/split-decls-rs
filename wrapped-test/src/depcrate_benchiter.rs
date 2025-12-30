// Generated macro for iter (function)
macro_rules! Depcrate_benchiter {
() => {
// Module: crate::bench
// Provides: {"iter"}
// Dependencies: {}
pub fn iter < T , F > (inner : & mut F) -> stats :: Summary where F : FnMut () -> T , { let ns_single = ns_iter_inner (inner , 1) ; let ns_target_total = 1_000_000 ; let mut n = ns_target_total / cmp :: max (1 , ns_single) ; n = cmp :: max (1 , n) ; let mut total_run = Duration :: new (0 , 0) ; let samples : & mut [f64] = & mut [0.0_f64 ; 50] ; loop { let loop_start = Instant :: now () ; for p in & mut * samples { * p = ns_iter_inner (inner , n) as f64 / n as f64 ; } stats :: winsorize (samples , 5.0) ; let summ = stats :: Summary :: new (samples) ; for p in & mut * samples { let ns = ns_iter_inner (inner , 5 * n) ; * p = ns as f64 / (5 * n) as f64 ; } stats :: winsorize (samples , 5.0) ; let summ5 = stats :: Summary :: new (samples) ; let loop_run = loop_start . elapsed () ; if loop_run > Duration :: from_millis (100) && summ . median_abs_dev_pct < 1.0 && summ . median - summ5 . median < summ5 . median_abs_dev { return summ5 ; } total_run += loop_run ; if total_run > Duration :: from_secs (3) { return summ5 ; } n = match n . checked_mul (10) { Some (_) => n * 2 , None => { return summ5 ; } } ; } }
};
}
