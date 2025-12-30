// Generated macro for load_values_to_names_sparse (function)
macro_rules! Depcrate_properties_enum_codepointtrieload_values_to_names_sparse {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"load_values_to_names_sparse"}
// Dependencies: {}
# [doc = " Load the mapping from property values to their names as a sparse map"] fn load_values_to_names_sparse < M > (p : & SourceDataProvider , prop_name : & str , is_short : bool ,) -> Result < DataResponse < M > , DataError > where M : DynamicDataMarker < DataStruct = PropertyEnumToValueNameSparseMap < 'static > > , { let data = p . get_enumerated_prop (prop_name) . map_err (| _ | DataError :: custom ("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)")) ? ; let map = load_values_to_names (data , is_short) ? ; let map = map . into_iter () . collect () ; let data_struct = PropertyEnumToValueNameSparseMap { map } ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data_struct) , }) }
};
}
