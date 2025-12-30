// Generated macro for sleep (function)
macro_rules! Depcratesleep {
() => {
// Module: crate
// Provides: {"sleep"}
// Dependencies: {}
pub fn sleep () { let duration = Duration :: from_millis (100) ; let now = Instant :: now () ; thread :: sleep (duration) ; let elapsed = now . elapsed () ; println ! ("Measured time for {duration:?} sleep: {elapsed:?}") ; }
};
}
