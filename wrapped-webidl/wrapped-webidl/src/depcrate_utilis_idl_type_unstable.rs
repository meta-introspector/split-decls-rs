// Generated macro for is_idl_type_unstable (function)
macro_rules! Depcrate_utilis_idl_type_unstable {
() => {
// Module: crate::util
// Provides: {"is_idl_type_unstable"}
// Dependencies: {}
fn is_idl_type_unstable (ty : & IdlType , unstable_types : & HashSet < Identifier >) -> bool { match ty { IdlType :: Identifier { ty : IdentifierType :: Dictionary (name) | IdentifierType :: Interface (name) , .. } => unstable_types . contains (& Identifier (name)) , _ => false , } }
};
}
