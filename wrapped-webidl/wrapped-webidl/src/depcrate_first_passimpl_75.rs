// Generated macro for impl_75 (impl)
macro_rules! Depcrate_first_passimpl_75 {
() => {
// Module: crate::first_pass
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: PartialNamespaceDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } for member in & self . members . body { member . first_pass (record , (self . identifier . 0 , stability)) ? ; } Ok (()) } }
};
}
