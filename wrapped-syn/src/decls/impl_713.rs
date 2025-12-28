macro_rules! deps {
    () => {
        IntoSpans!();
        Group!();
    };
}

macro_rules! impl_713 {
    () => {
        deps!();
        impl IntoSpans < DelimSpan > for Span { fn into_spans (self) -> DelimSpan { let mut group = Group :: new (Delimiter :: None , TokenStream :: new ()) ; group . set_span (self) ; group . delim_span () } }
    };
}

impl_713!()