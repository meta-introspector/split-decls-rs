// Generated macro for make_os_string (macro)
macro_rules! Depcrate_compiler_rustmake_os_string {
() => {
// Module: crate::compiler::rust
// Provides: {"make_os_string"}
// Dependencies: {}
macro_rules ! make_os_string { ($ ($ v : expr) ,*) => { { let mut s = OsString :: new () ; $ (s . push ($ v) ;) * s } } ; }
};
}
