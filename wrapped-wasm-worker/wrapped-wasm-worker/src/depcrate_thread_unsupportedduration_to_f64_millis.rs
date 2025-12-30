// Generated macro for duration_to_f64_millis (function)
macro_rules! Depcrate_thread_unsupportedduration_to_f64_millis {
() => {
// Module: crate::thread::unsupported
// Provides: {"duration_to_f64_millis"}
// Dependencies: {}
# [doc = " Converts [`Duration`] to amount of milliseconds as [`f64`]."] fn duration_to_f64_millis (duration : Duration) -> f64 { duration . checked_mul (1000) . map_or (f64 :: INFINITY , | duration | duration . as_secs_f64 ()) }
};
}
