macro_rules! deps {
    () => {
        SyntaxContextData!();
        SyntaxContext!();
        SyntaxContextKey!();
        Transparency!();
        ExpnId!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl SyntaxContextData { fn root () -> SyntaxContextData { SyntaxContextData { outer_expn : ExpnId :: root () , outer_transparency : Transparency :: Opaque , parent : SyntaxContext :: root () , opaque : SyntaxContext :: root () , opaque_and_semiopaque : SyntaxContext :: root () , dollar_crate_name : kw :: DollarCrate , } } fn key (& self) -> SyntaxContextKey { (self . parent , self . outer_expn , self . outer_transparency) } }
    };
}

impl_41!()