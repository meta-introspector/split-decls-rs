// Generated macro for forward_deserialize_methods (macro)
macro_rules! Depcrate_configureforward_deserialize_methods {
() => {
// Module: crate::configure
// Provides: {"forward_deserialize_methods"}
// Dependencies: {}
macro_rules ! forward_deserialize_methods { ($ wrapper : ident ($ ($ name : ident) ,*)) => { $ (fn $ name < V > (self , visitor : V) -> Result < V :: Value , D :: Error > where V : Visitor <'de >, { (self . 0) .$ name ($ wrapper (visitor)) }) * } ; }
};
}
