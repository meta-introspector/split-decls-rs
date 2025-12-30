// Generated macro for time (function)
macro_rules! Depcratetime {
() => {
// Module: crate
// Provides: {"time"}
// Dependencies: {}
fn time < F , T > (time_out : & mut f64 , mut f : F) -> T where F : FnMut () -> T , { let start = Instant :: now () ; let r = f () ; let end = Instant :: now () ; * time_out += duration_nanos (end . duration_since (start)) ; r }
};
}
