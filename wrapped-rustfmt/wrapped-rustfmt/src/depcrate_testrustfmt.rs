// Generated macro for rustfmt (function)
macro_rules! Depcrate_testrustfmt {
() => {
// Module: crate::test
// Provides: {"rustfmt"}
// Dependencies: {}
fn rustfmt () -> PathBuf { let mut me = env :: current_exe () . expect ("failed to get current executable") ; me . pop () ; me . pop () ; me . push ("rustfmt") ; assert ! (me . is_file () || me . with_extension ("exe") . is_file () , "{}" , "no rustfmt bin, try running `cargo build` or `cargo build --release` before testing") ; me }
};
}
