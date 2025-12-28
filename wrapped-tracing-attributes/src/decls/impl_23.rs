macro_rules! deps {
    () => {
        FieldKind!();
        Field!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl ToTokens for Field { fn to_tokens (& self , tokens : & mut TokenStream) { if let Some (ref value) = self . value { let name = & self . name ; let kind = & self . kind ; tokens . extend (quote ! { # name = # kind # value }) } else if self . kind == FieldKind :: Value { let name = & self . name ; tokens . extend (quote ! (# name = :: tracing :: field :: Empty)) } else { self . kind . to_tokens (tokens) ; self . name . to_tokens (tokens) ; } } }
    };
}

impl_23!()