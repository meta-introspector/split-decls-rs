// Generated macro for impl_63 (impl)
macro_rules! Depcrate_instrumentimpl_63 {
() => {
// Module: crate::instrument
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , T > InstrumentedProj < 'a , T > { # [doc = " Get a mutable reference to the [`Span`] a pinned mutable reference to"] # [doc = " the wrapped type."] fn span_and_inner_pin_mut (self) -> (& 'a mut Span , Pin < & 'a mut T >) { let inner = unsafe { self . inner . map_unchecked_mut (| v | & mut * * v) } ; (self . span , inner) } }
};
}
