macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl < T > Instrumented < T > { # [doc = " Borrows the `Span` that this type is instrumented by."] pub fn span (& self) -> & Span { & self . span } # [doc = " Mutably borrows the `Span` that this type is instrumented by."] pub fn span_mut (& mut self) -> & mut Span { & mut self . span } # [doc = " Borrows the wrapped type."] pub fn inner (& self) -> & T { & self . inner } # [doc = " Mutably borrows the wrapped type."] pub fn inner_mut (& mut self) -> & mut T { & mut self . inner } # [doc = " Get a pinned reference to the wrapped type."] pub fn inner_pin_ref (self : Pin < & Self >) -> Pin < & T > { self . project_ref () . span_and_inner_pin_ref () . 1 } # [doc = " Get a pinned mutable reference to the wrapped type."] pub fn inner_pin_mut (self : Pin < & mut Self >) -> Pin < & mut T > { self . project () . span_and_inner_pin_mut () . 1 } # [doc = " Consumes the `Instrumented`, returning the wrapped type."] # [doc = ""] # [doc = " Note that this drops the span."] pub fn into_inner (self) -> T { let this = ManuallyDrop :: new (self) ; let span : * const Span = & this . span ; let inner : * const ManuallyDrop < T > = & this . inner ; let _span = unsafe { span . read () } ; let inner = unsafe { inner . read () } ; ManuallyDrop :: into_inner (inner) } }
    };
}

impl_47!();