// Generated macro for RustcInternal (trait)
macro_rules! Depcrate_unstableRustcInternal {
() => {
// Module: crate::unstable
// Provides: {"RustcInternal"}
// Dependencies: {}
# [doc = " Trait used to translate a rustc_public's IR construct to its rustc counterpart."] # [doc = ""] # [doc = " This is basically a mirror of [Stable]."] # [doc = ""] # [doc = " This trait is currently exposed to users so they can have interoperability"] # [doc = " between internal MIR and rustc_public's IR constructs."] # [doc = " They should be used seldom as they have no stability guarantees."] # [doc (hidden)] pub trait RustcInternal { type T < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > ; }
};
}
