// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> anyhow :: Result < () > { let cli = cli () ; exec (& cli . get_matches ()) ? ; Ok (()) }
};
}
