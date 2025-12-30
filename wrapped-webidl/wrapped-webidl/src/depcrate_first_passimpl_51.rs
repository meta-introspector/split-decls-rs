// Generated macro for impl_51 (impl)
macro_rules! Depcrate_first_passimpl_51 {
() => {
// Module: crate::first_pass
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: DictionaryDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } let dictionary_data = record . dictionaries . entry (self . identifier . 0) . or_default () ; dictionary_data . definition = Some (self) ; dictionary_data . stability = stability ; Ok (()) } }
};
}
