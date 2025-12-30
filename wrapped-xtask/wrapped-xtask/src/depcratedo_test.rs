// Generated macro for do_test (function)
macro_rules! Depcratedo_test {
() => {
// Module: crate
// Provides: {"do_test"}
// Dependencies: {}
fn do_test (test : impl FnOnce () -> anyhow :: Result < () > , context : & str) { test () . unwrap_or_else (| e | ALL_ERRORS . lock () . unwrap () . push (format ! ("{context}: {e}"))) ; }
};
}
