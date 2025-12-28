macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_717 {
    () => {
        deps!();
        impl < T : ? Sized + ToTokens > Spanned for T { fn span (& self) -> Span { self . __span () } }
    };
}

impl_717!();