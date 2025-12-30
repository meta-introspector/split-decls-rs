// Generated macro for forward_method (macro)
macro_rules! Depcrate_configureforward_method {
() => {
// Module: crate::configure
// Provides: {"forward_method"}
// Dependencies: {}
macro_rules ! forward_method { ($ name : ident (self $ (, $ arg : ident : $ arg_type : ty) *) -> $ return_type : ty) => { fn $ name (self $ (, $ arg : $ arg_type) *) -> $ return_type { (self . 0) .$ name ($ ($ arg) ,*) } } ; }
};
}
