// Generated macro for impl_52 (impl)
macro_rules! Depcrate_first_passimpl_52 {
() => {
// Module: crate::first_pass
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: PartialDictionaryDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } record . dictionaries . entry (self . identifier . 0) . or_default () . partials . push (PartialDictionaryData { definition : self , stability , }) ; Ok (()) } }
};
}
