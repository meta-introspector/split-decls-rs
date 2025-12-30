// Generated macro for impl_74 (impl)
macro_rules! Depcrate_first_passimpl_74 {
() => {
// Module: crate::first_pass
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: NamespaceDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } let namespace = record . namespaces . entry (self . identifier . 0) . or_default () ; namespace . stability = stability ; for member in & self . members . body { member . first_pass (record , (self . identifier . 0 , stability)) ? ; } Ok (()) } }
};
}
