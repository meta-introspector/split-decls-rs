// Generated macro for join_thread (function)
macro_rules! Depcratejoin_thread {
() => {
// Module: crate
// Provides: {"join_thread"}
// Dependencies: {}
fn join_thread () { let mut writer = STATE . write () . unwrap () ; let thread = writer . thread . take () ; drop (writer) ; thread . unwrap () . join () . unwrap () ; }
};
}
