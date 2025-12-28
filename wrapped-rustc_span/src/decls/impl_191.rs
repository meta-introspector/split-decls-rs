macro_rules! deps {
    () => {
        Macros20NormalizedIdent!();
        Ident!();
        Symbol!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl Macros20NormalizedIdent { # [inline] pub fn new (ident : Ident) -> Self { Macros20NormalizedIdent (ident . normalize_to_macros_2_0 ()) } pub fn with_dummy_span (name : Symbol) -> Self { Macros20NormalizedIdent (Ident :: with_dummy_span (name)) } }
    };
}

impl_191!()