// Generated macro for opaque_ty (function)
macro_rules! Depcrate_typesopaque_ty {
() => {
// Module: crate::types
// Provides: {"opaque_ty"}
// Dependencies: {}
pub (crate) fn opaque_ty (ty : & Option < ptr :: P < ast :: Ty > >) -> Option < & ast :: GenericBounds > { ty . as_ref () . and_then (| t | match & t . kind { ast :: TyKind :: ImplTrait (_ , bounds) => Some (bounds) , _ => None , }) }
};
}
