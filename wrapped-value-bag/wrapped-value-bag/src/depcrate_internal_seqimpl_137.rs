// Generated macro for impl_137 (impl)
macro_rules! Depcrate_internal_seqimpl_137 {
() => {
// Module: crate::internal::seq
// Provides: {"impl_137"}
// Dependencies: {}
impl < 'v , S : ExtendValue < 'v > > Visitor < 'v > for ExtendVisitor < S > { fn element (& mut self , v : ValueBag) -> ControlFlow < () > { self . 0 . extend (v . inner) ; ControlFlow :: Continue (()) } fn borrowed_element (& mut self , v : ValueBag < 'v >) -> ControlFlow < () > { self . 0 . extend_borrowed (v . inner) ; ControlFlow :: Continue (()) } }
};
}
