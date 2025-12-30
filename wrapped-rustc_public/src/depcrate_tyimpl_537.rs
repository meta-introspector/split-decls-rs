// Generated macro for impl_537 (impl)
macro_rules! Depcrate_tyimpl_537 {
() => {
// Module: crate::ty
// Provides: {"impl_537"}
// Dependencies: {}
impl GenericArgKind { # [doc = " Panic if this generic argument is not a type, otherwise"] # [doc = " return the type."] # [track_caller] pub fn expect_ty (& self) -> & Ty { match self { GenericArgKind :: Type (ty) => ty , _ => panic ! ("{self:?}") , } } # [doc = " Panic if this generic argument is not a const, otherwise"] # [doc = " return the const."] # [track_caller] pub fn expect_const (& self) -> & TyConst { match self { GenericArgKind :: Const (c) => c , _ => panic ! ("{self:?}") , } } # [doc = " Return the generic argument type if applicable, otherwise return `None`."] pub fn ty (& self) -> Option < & Ty > { match self { GenericArgKind :: Type (ty) => Some (ty) , _ => None , } } }
};
}
