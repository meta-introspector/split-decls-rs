macro_rules! deps {
    () => {
        Span!();
        Event!();
        EventKind!();
        Encoding!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl Event { pub fn new_unchecked (kind : EventKind , encoding : Option < Encoding > , span : Span) -> Self { Self { kind , encoding , span , } } # [inline (always)] pub fn kind (& self) -> EventKind { self . kind } # [inline (always)] pub fn encoding (& self) -> Option < Encoding > { self . encoding } # [inline (always)] pub fn span (& self) -> Span { self . span } }
    };
}

impl_190!();