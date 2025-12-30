// Generated macro for Stable (trait)
macro_rules! Depcrate_unstableStable {
() => {
// Module: crate::unstable
// Provides: {"Stable"}
// Dependencies: {}
# [doc = " Trait used to convert between an internal MIR type to a rustc_public's IR type."] # [doc = ""] # [doc = " This trait is currently exposed to users so they can have interoperability"] # [doc = " between internal MIR and rustc_public's IR constructs."] # [doc = " However, they should be used seldom and they have no influence in this crate semver."] # [doc (hidden)] pub trait Stable < 'tcx > : PointeeSized { # [doc = " The stable representation of the type implementing Stable."] type T ; # [doc = " Converts an object to the equivalent rustc_public's IR representation."] fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T ; }
};
}
