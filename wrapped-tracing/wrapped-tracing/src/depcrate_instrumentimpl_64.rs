// Generated macro for impl_64 (impl)
macro_rules! Depcrate_instrumentimpl_64 {
() => {
// Module: crate::instrument
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'a , T > InstrumentedProjRef < 'a , T > { # [doc = " Get a reference to the [`Span`] a pinned reference to the wrapped type."] fn span_and_inner_pin_ref (self) -> (& 'a Span , Pin < & 'a T >) { let inner = unsafe { self . inner . map_unchecked (| v | & * * v) } ; (self . span , inner) } }
};
}
