// Generated macro for forward_serialize_methods (macro)
macro_rules! Depcrate_configureforward_serialize_methods {
() => {
// Module: crate::configure
// Provides: {"forward_serialize_methods"}
// Dependencies: {}
macro_rules ! forward_serialize_methods { ($ ($ name : ident $ arg_type : ty) ,*) => { $ (forward_method ! ($ name (self , v : $ arg_type) -> Result < Self :: Ok , Self :: Error >) ;) * } ; }
};
}
