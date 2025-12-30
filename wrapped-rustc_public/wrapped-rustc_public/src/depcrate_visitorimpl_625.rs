// Generated macro for impl_625 (impl)
macro_rules! Depcrate_visitorimpl_625 {
() => {
// Module: crate::visitor
// Provides: {"impl_625"}
// Dependencies: {}
impl < T : Visitable > Visitable for Option < T > { fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { match self { Some (val) => val . visit (visitor) , None => ControlFlow :: Continue (()) , } } }
};
}
