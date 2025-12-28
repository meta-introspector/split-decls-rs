macro_rules! deps {
    () => {
        Ident!();
        MacroRulesNormalizedIdent!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl MacroRulesNormalizedIdent { # [inline] pub fn new (ident : Ident) -> Self { MacroRulesNormalizedIdent (ident . normalize_to_macro_rules ()) } }
    };
}

impl_187!()