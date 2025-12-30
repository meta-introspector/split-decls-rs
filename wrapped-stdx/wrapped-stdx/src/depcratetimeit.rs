// Generated macro for timeit (function)
macro_rules! Depcratetimeit {
() => {
// Module: crate
// Provides: {"timeit"}
// Dependencies: {}
# [must_use] # [expect (clippy :: print_stderr , reason = "only visible to developers")] pub fn timeit (label : & 'static str) -> impl Drop { let start = Instant :: now () ; defer (move | | eprintln ! ("{}: {:.2}" , label , start . elapsed () . as_nanos ())) }
};
}
