macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! Macros20NormalizedIdent {
    () => {
        deps!();
        # [doc = " An newtype around `Ident` that calls [Ident::normalize_to_macros_2_0] on"] # [doc = " construction for \"item hygiene\" comparisons."] # [doc = ""] # [doc = " Identifiers with same string value become same if they came from the same macro 2.0 macro"] # [doc = " (e.g., `macro` item, but not `macro_rules` item) and stay different if they came from"] # [doc = " different macro 2.0 macros."] # [derive (Copy , Clone , Eq , PartialEq , Hash)] pub struct Macros20NormalizedIdent (pub Ident) ;
    };
}

Macros20NormalizedIdent!()