// Generated macro for expand (macro)
macro_rules! Depcrate_properties_emoji_setexpand {
() => {
// Module: crate::properties::emoji_set
// Provides: {"expand"}
// Dependencies: {}
macro_rules ! expand { ($ (($ marker : ident , $ prop_name : literal)) ,+) => { $ (impl DataProvider <$ marker > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse <$ marker >, DataError > { self . check_req ::<$ marker > (req) ?; let data = self . get_binary_prop_for_unicodeset ($ prop_name) ?; let mut builder = CodePointInversionListBuilder :: new () ; for (start , end) in & data . ranges { builder . add_range32 (start ..= end) ; } let inv_list = builder . build () ; let strings = data . strings . as_ref () . ok_or (DataError :: custom ("Error in deserializing strings from BinaryProperty source data")) ?; let string_list = VarZeroVec ::< str >:: from (strings) ; let uniset = CodePointInversionListAndStringList :: try_from (inv_list , string_list) . map_err (| _ | DataError :: custom ("Error in constructing CodePointInversionListAndStringList from deserialized BinaryProperty data")) ?; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (PropertyUnicodeSet :: CPInversionListStrList (uniset) ,) , }) } } impl crate :: IterableDataProviderCached <$ marker > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow <'static >>, DataError > { self . get_binary_prop_for_unicodeset ($ prop_name) ?; Ok (HashSet :: from_iter ([Default :: default ()])) } }) + } ; }
};
}
