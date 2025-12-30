// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "std-future")] impl < 'a , T > InstrumentedProj < 'a , T > { # [doc = " Get a mutable reference to the [`Span`] a pinned mutable reference to"] # [doc = " the wrapped type."] fn span_and_inner_pin_mut (self) -> (& 'a mut Span , Pin < & 'a mut T >) { let inner = unsafe { self . inner . map_unchecked_mut (| v | & mut * * v) } ; (self . span , inner) } }
};
}
