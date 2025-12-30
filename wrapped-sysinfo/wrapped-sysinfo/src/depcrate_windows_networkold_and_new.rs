// Generated macro for old_and_new (macro)
macro_rules! Depcrate_windows_networkold_and_new {
() => {
// Module: crate::windows::network
// Provides: {"old_and_new"}
// Dependencies: {}
macro_rules ! old_and_new { ($ ty_ : expr , $ name : ident , $ old : ident , $ new_val : expr) => { { $ ty_ .$ old = $ ty_ .$ name ; $ ty_ .$ name = $ new_val ; } } ; }
};
}
