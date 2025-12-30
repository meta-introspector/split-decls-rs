// Generated macro for old_and_new (macro)
macro_rules! Depcrate_unix_linux_networkold_and_new {
() => {
// Module: crate::unix::linux::network
// Provides: {"old_and_new"}
// Dependencies: {}
macro_rules ! old_and_new { ($ ty_ : expr , $ name : ident , $ old : ident) => { { $ ty_ .$ old = $ ty_ .$ name ; $ ty_ .$ name = $ name ; } } ; ($ ty_ : expr , $ name : ident , $ old : ident , $ path : expr) => { { let _tmp = $ path ; $ ty_ .$ old = $ ty_ .$ name ; $ ty_ .$ name = _tmp ; } } ; }
};
}
