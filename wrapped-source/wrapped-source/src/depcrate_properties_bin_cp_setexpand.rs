// Generated macro for expand (macro)
macro_rules! Depcrate_properties_bin_cp_setexpand {
() => {
// Module: crate::properties::bin_cp_set
// Provides: {"expand"}
// Dependencies: {}
macro_rules ! expand { ($ (($ marker : ident , $ prop_name : literal)) ,+) => { $ (impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; let data = self . get_binary_prop_for_code_point_set ($ prop_name) ?; let mut builder = CodePointInversionListBuilder :: new () ; for (start , end) in & data . ranges { builder . add_range32 (start ..= end) ; } let inv_list = builder . build () ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (PropertyCodePointSet :: InversionList (inv_list) ,) , }) } } impl crate :: IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { self . get_binary_prop_for_code_point_set ($ prop_name) ?; Ok (HashSet :: from_iter ([Default :: default ()])) } }) + } ; }
};
}
