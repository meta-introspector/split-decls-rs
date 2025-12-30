// Generated macro for should_exit (function)
macro_rules! Depcrateshould_exit {
() => {
// Module: crate
// Provides: {"should_exit"}
// Dependencies: {}
fn should_exit (events : & [Event]) -> bool { events . iter () . any (| event | matches ! (event , Event :: Key (key) if key . code == KeyCode :: Char ('q'))) }
};
}
