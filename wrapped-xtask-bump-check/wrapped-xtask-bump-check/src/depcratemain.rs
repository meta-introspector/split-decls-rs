// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { setup_logger () ; let cli = xtask :: cli () ; let matches = cli . get_matches () ; let mut gctx = cargo :: util :: context :: GlobalContext :: default () . unwrap_or_else (| e | { let mut eval = cargo :: core :: shell :: Shell :: new () ; cargo :: exit_with_error (e . into () , & mut eval) }) ; if let Err (e) = xtask :: exec (& matches , & mut gctx) { cargo :: exit_with_error (e , & mut gctx . shell ()) } }
};
}
