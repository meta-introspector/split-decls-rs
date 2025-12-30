// Generated macro for DECLARE_HANDLE (macro)
macro_rules! Depcrate_macrosDECLARE_HANDLE {
() => {
// Module: crate::macros
// Provides: {"DECLARE_HANDLE"}
// Dependencies: {}
macro_rules ! DECLARE_HANDLE { ($ name : ident , $ inner : ident) => { pub enum $ inner { } pub type $ name = * mut $ inner ; } ; }
};
}
