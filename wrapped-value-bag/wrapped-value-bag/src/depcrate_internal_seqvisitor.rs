// Generated macro for Visitor (trait)
macro_rules! Depcrate_internal_seqVisitor {
() => {
// Module: crate::internal::seq
// Provides: {"Visitor"}
// Dependencies: {}
pub (crate) trait Visitor < 'v > { fn element (& mut self , v : ValueBag) -> ControlFlow < () > ; fn borrowed_element (& mut self , v : ValueBag < 'v >) -> ControlFlow < () > { self . element (v) } }
};
}
