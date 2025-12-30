// Generated macro for is_type_unstable (function)
macro_rules! Depcrate_utilis_type_unstable {
() => {
// Module: crate::util
// Provides: {"is_type_unstable"}
// Dependencies: {}
pub fn is_type_unstable (ty : & weedle :: types :: Type , unstable_types : & HashSet < Identifier >) -> bool { match ty { weedle :: types :: Type :: Single (SingleType :: NonAny (NonAnyType :: Identifier (i))) => { unstable_types . contains (& i . type_) } _ => false , } }
};
}
