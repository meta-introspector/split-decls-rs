// Generated macro for impl_633 (impl)
macro_rules! Depcrate_visitorimpl_633 {
() => {
// Module: crate::visitor
// Provides: {"impl_633"}
// Dependencies: {}
impl Visitable for ExistentialPredicate { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { ExistentialPredicate :: Trait (tr) => tr . generic_args . visit (visitor) , ExistentialPredicate :: Projection (p) => { p . term . visit (visitor) ? ; p . generic_args . visit (visitor) } ExistentialPredicate :: AutoTrait (_) => ControlFlow :: Continue (()) , } } }
};
}
