// Generated macro for load_values_to_names_linear (function)
macro_rules! Depcrate_properties_enum_codepointtrieload_values_to_names_linear {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"load_values_to_names_linear"}
// Dependencies: {}
# [doc = " Load the mapping from property values to their names as a linear map"] fn load_values_to_names_linear < M > (p : & SourceDataProvider , prop_name : & str , is_short : bool ,) -> Result < DataResponse < M > , DataError > where M : DynamicDataMarker < DataStruct = PropertyEnumToValueNameLinearMap < 'static > > , { let data = p . get_enumerated_prop (prop_name) . map_err (| _ | DataError :: custom ("Loading icuexport property data failed: \
                                        Are you using a sufficiently recent icuexport? (Must be ⪈ 72.1)")) ? ; let map = load_values_to_names (data , is_short) ? ; let vec = map_to_vec (& map , prop_name) ? ; let varzerovec = (& vec) . into () ; let data_struct = PropertyEnumToValueNameLinearMap { map : varzerovec } ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data_struct) , }) }
};
}
