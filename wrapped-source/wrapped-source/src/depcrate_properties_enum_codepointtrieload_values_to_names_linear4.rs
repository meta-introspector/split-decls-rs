// Generated macro for load_values_to_names_linear4 (function)
macro_rules! Depcrate_properties_enum_codepointtrieload_values_to_names_linear4 {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"load_values_to_names_linear4"}
// Dependencies: {}
# [doc = " Load the mapping from property values to their names as a linear map of TinyStr4s"] fn load_values_to_names_linear4 < M > (p : & SourceDataProvider , prop_name : & str , is_short : bool ,) -> Result < DataResponse < M > , DataError > where M : DynamicDataMarker < DataStruct = PropertyScriptToIcuScriptMap < 'static > > , { let data = p . get_enumerated_prop (prop_name) . map_err (| _ | DataError :: custom ("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)")) ? ; let map = load_values_to_names (data , is_short) ? ; let vec = map_to_vec (& map , prop_name) ? ; let vec : Result < Vec < _ > , _ > = vec . into_iter () . map (| s | { if s . is_empty () { Ok (None) } else { icu :: locale :: subtags :: Script :: try_from_str (s) . map (Some) } }) . collect () ; let vec = vec . map_err (| _ | DataError :: custom ("Found invalid script tag")) ? ; let zerovec = vec . into_iter () . map (NichedOption) . collect () ; let data_struct = PropertyScriptToIcuScriptMap { map : zerovec } ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data_struct) , }) }
};
}
