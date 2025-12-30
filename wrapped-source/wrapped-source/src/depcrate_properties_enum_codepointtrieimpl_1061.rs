// Generated macro for impl_1061 (impl)
macro_rules! Depcrate_properties_enum_codepointtrieimpl_1061 {
() => {
// Module: crate::properties::enum_codepointtrie
// Provides: {"impl_1061"}
// Dependencies: {}
impl DataProvider < PropertyNameParseGeneralCategoryMaskV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < PropertyNameParseGeneralCategoryMaskV1 > , DataError > { use icu :: properties :: props :: GeneralCategoryGroup ; use zerovec :: ule :: AsULE ; self . check_req :: < PropertyNameParseGeneralCategoryMaskV1 > (req) ? ; let data = self . get_mask_prop ("gcm") ? ; let data_struct = get_prop_values_map (& data . values , | v | { let value : GeneralCategoryGroup = v . into () ; let ule = value . to_unaligned () ; let packed = u16 :: from_unaligned (ule) ; if packed == 0xFF00 { return Err (DataError :: custom ("Found unknown general category mask value, properties code may need to be updated.")) ; } Ok (packed) }) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (data_struct) , }) } }
};
}
