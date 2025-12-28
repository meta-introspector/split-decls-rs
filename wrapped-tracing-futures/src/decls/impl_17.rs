macro_rules! deps {
    () => {
        Instrumented!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < T > Instrumented < T > { # [doc = " Borrows the `Span` that this type is instrumented by."] pub fn span (& self) -> & Span { & self . span } # [doc = " Mutably borrows the `Span` that this type is instrumented by."] pub fn span_mut (& mut self) -> & mut Span { & mut self . span } # [doc = " Borrows the wrapped type."] pub fn inner (& self) -> & T { & self . inner } # [doc = " Mutably borrows the wrapped type."] pub fn inner_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Get a pinned reference to the wrapped type."] # [cfg (feature = "std-future")] # [cfg_attr (docsrs , doc (cfg (feature = "std-future")))] pub fn inner_pin_ref (self : Pin < & Self >) -> Pin < & T > { self . project_ref () . span_and_inner_pin_ref () . 1 } # [doc = " Get a pinned mutable reference to the wrapped type."] # [cfg (feature = "std-future")] # [cfg_attr (docsrs , doc (cfg (feature = "std-future")))] pub fn inner_pin_mut (self : Pin < & mut Self >) -> Pin < & mut T > { self . project () . span_and_inner_pin_mut () . 1 } # [doc = " Consumes the `Instrumented`, returning the wrapped type."] # [doc = ""] # [doc = " Note that this drops the span."] pub fn into_inner (self) -> T { # [cfg (feature = "std-future")] { let span : * const Span = & self . span ; let inner : * const ManuallyDrop < T > = & self . inner ; mem :: forget (self) ; let _span = unsafe { span . read () } ; let inner = unsafe { inner . read () } ; ManuallyDrop :: into_inner (inner) } # [cfg (not (feature = "std-future"))] self . inner } }
    };
}

impl_17!()