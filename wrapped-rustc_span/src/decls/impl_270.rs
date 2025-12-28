macro_rules! deps {
    () => {
        SpanEncoder!();
        Symbol!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < E : SpanEncoder > Encodable < E > for Symbol { fn encode (& self , s : & mut E) { s . encode_symbol (* self) ; } }
    };
}

impl_270!();