macro_rules! deps {
    () => {
        LitStrOrIdent!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Parse for LitStrOrIdent { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { input . parse :: < LitStr > () . map (LitStrOrIdent :: LitStr) . or_else (| _ | input . parse :: < Ident > () . map (LitStrOrIdent :: Ident)) } }
    };
}

impl_9!()