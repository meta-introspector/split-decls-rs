macro_rules! deps {
    () => {
        MacroRulesNormalizedIdent!();
        Ident!();
    };
}

macro_rules! impl_187 {
    () => {
        deps!();
        impl MacroRulesNormalizedIdent { # [inline] pub fn new (ident : Ident) -> Self { MacroRulesNormalizedIdent (ident . normalize_to_macro_rules ()) } }
    };
}

impl_187!();