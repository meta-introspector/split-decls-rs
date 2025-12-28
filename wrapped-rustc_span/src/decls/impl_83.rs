macro_rules! deps {
    () => {
        SpanEncoder!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for LocalExpnId { fn encode (& self , e : & mut E) { self . to_expn_id () . encode (e) ; } }
    };
}

impl_83!();