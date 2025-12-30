// Generated macro for TypeSuperVisitable (trait)
macro_rules! Depcrate_visitTypeSuperVisitable {
() => {
// Module: crate::visit
// Provides: {"TypeSuperVisitable"}
// Dependencies: {}
pub trait TypeSuperVisitable < I : Interner > : TypeVisitable < I > { # [doc = " Provides a default visit for a recursive type of interest. This should"] # [doc = " only be called within `TypeVisitor` methods, when a non-custom"] # [doc = " traversal is desired for the value of the type of interest passed to"] # [doc = " that method. For example, in `MyVisitor::visit_ty(ty)`, it is valid to"] # [doc = " call `ty.super_visit_with(self)`, but any other visiting should be done"] # [doc = " with `xyz.visit_with(self)`."] fn super_visit_with < V : TypeVisitor < I > > (& self , visitor : & mut V) -> V :: Result ; }
};
}
