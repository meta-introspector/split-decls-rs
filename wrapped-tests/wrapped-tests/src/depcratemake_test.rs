// Generated macro for make_test (macro)
macro_rules! Depcratemake_test {
() => {
// Module: crate
// Provides: {"make_test"}
// Dependencies: {}
# [macro_export] macro_rules ! make_test { ($ ($ test_name : ident ($ ($ arg : expr) ,* $ (,) ?)) ,* $ (,) ?) => { $ (# [test] fn $ test_name () { $ crate ::$ test_name ($ ($ arg) ,*) ; }) * } ; }
};
}
