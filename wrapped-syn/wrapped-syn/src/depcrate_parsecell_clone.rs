// Generated macro for cell_clone (function)
macro_rules! Depcrate_parsecell_clone {
() => {
// Module: crate::parse
// Provides: {"cell_clone"}
// Dependencies: {}
fn cell_clone < T : Default + Clone > (cell : & Cell < T >) -> T { let prev = cell . take () ; let ret = prev . clone () ; cell . set (prev) ; ret }
};
}
