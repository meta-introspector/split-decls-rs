macro_rules! deps {
    () => {
        SpanMapper!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < S > SpanMapper < SpanData < S > > for SpanMap < S > where SpanData < S > : Copy , { fn span_for (& self , range : TextRange) -> SpanData < S > { self . span_at (range . start ()) } }
    };
}

impl_4!()