macro_rules! impl_15 {
    () => {
        # [cfg (feature = "std-future")] impl < 'a , T > InstrumentedProj < 'a , T > { # [doc = " Get a mutable reference to the [`Span`] a pinned mutable reference to"] # [doc = " the wrapped type."] fn span_and_inner_pin_mut (self) -> (& 'a mut Span , Pin < & 'a mut T >) { let inner = unsafe { self . inner . map_unchecked_mut (| v | & mut * * v) } ; (self . span , inner) } }
    };
}

impl_15!()