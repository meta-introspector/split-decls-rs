macro_rules! deps {
    () => {
        Delimiter!();
        DelimiterKind!();
        DelimSpan!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < S : Copy > Delimiter < S > { pub const fn invisible_spanned (span : S) -> Self { Delimiter { open : span , close : span , kind : DelimiterKind :: Invisible } } pub const fn invisible_delim_spanned (span : DelimSpan < S >) -> Self { Delimiter { open : span . open , close : span . close , kind : DelimiterKind :: Invisible } } pub fn delim_span (& self) -> DelimSpan < S > { DelimSpan { open : self . open , close : self . close } } }
    };
}

impl_33!()