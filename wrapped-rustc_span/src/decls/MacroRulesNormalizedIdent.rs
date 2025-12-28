macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! MacroRulesNormalizedIdent {
    () => {
        deps!();
        # [doc = " An newtype around `Ident` that calls [Ident::normalize_to_macro_rules] on"] # [doc = " construction for \"local variable hygiene\" comparisons."] # [doc = ""] # [doc = " Use this type when you need to compare identifiers according to macro_rules hygiene."] # [doc = " This ensures compile-time safety and avoids manual normalization calls."] # [derive (Copy , Clone , Eq , PartialEq , Hash)] pub struct MacroRulesNormalizedIdent (Ident) ;
    };
}

MacroRulesNormalizedIdent!();