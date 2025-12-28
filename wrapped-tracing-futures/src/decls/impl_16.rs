macro_rules! impl_16 {
    () => {
        # [cfg (feature = "std-future")] impl < 'a , T > InstrumentedProjRef < 'a , T > { # [doc = " Get a reference to the [`Span`] a pinned reference to the wrapped type."] fn span_and_inner_pin_ref (self) -> (& 'a Span , Pin < & 'a T >) { let inner = unsafe { self . inner . map_unchecked (| v | & * * v) } ; (self . span , inner) } }
    };
}

impl_16!();