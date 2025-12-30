// Generated macro for impl_120 (impl)
macro_rules! Depcrate_internal_seqimpl_120 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'a , 'v , T : Visitor < 'v > + ? Sized > Visitor < 'v > for & 'a mut T { fn element (& mut self , v : ValueBag) -> ControlFlow < () > { (* * self) . element (v) } fn borrowed_element (& mut self , v : ValueBag < 'v >) -> ControlFlow < () > { (* * self) . borrowed_element (v) } }
};
}
