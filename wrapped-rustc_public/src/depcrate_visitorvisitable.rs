// Generated macro for Visitable (trait)
macro_rules! Depcrate_visitorVisitable {
() => {
// Module: crate::visitor
// Provides: {"Visitable"}
// Dependencies: {}
pub trait Visitable { fn visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > { self . super_visit (visitor) } fn super_visit < V : Visitor > (& self , visitor : & mut V) -> ControlFlow < V :: Break > ; }
};
}
