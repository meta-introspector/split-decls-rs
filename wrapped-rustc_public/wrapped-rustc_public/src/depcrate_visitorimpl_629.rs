// Generated macro for impl_629 (impl)
macro_rules! Depcrate_visitorimpl_629 {
() => {
// Module: crate::visitor
// Provides: {"impl_629"}
// Dependencies: {}
impl Visitable for GenericArgKind { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { GenericArgKind :: Lifetime (lt) => lt . visit (visitor) , GenericArgKind :: Type (t) => t . visit (visitor) , GenericArgKind :: Const (c) => c . visit (visitor) , } } }
};
}
