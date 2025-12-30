// Generated macro for impl_619 (impl)
macro_rules! Depcrate_visitorimpl_619 {
() => {
// Module: crate::visitor
// Provides: {"impl_619"}
// Dependencies: {}
impl Visitable for TermKind { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { TermKind :: Type (t) => t . visit (visitor) , TermKind :: Const (c) => c . visit (visitor) , } } }
};
}
