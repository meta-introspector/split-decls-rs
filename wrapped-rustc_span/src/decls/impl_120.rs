macro_rules! deps {
    () => {
        LocalDefId!();
        SpanEncoder!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for LocalDefId { fn encode (& self , s : & mut E) { self . to_def_id () . encode (s) ; } }
    };
}

impl_120!();