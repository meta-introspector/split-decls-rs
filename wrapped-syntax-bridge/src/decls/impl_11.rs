macro_rules! deps {
    () => {
        SpanMapper!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < S : Copy , SM : SpanMapper < S > > SpanMapper < S > for & SM { fn span_for (& self , range : TextRange) -> S { SM :: span_for (self , range) } }
    };
}

impl_11!();