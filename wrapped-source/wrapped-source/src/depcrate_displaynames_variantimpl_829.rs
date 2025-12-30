// Generated macro for impl_829 (impl)
macro_rules! Depcrate_displaynames_variantimpl_829 {
() => {
// Module: crate::displaynames::variant
// Provides: {"impl_829"}
// Dependencies: {}
impl DataProvider < VariantDisplayNamesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < VariantDisplayNamesV1 > , DataError > { self . check_req :: < VariantDisplayNamesV1 > (req) ? ; let data : & cldr_serde :: displaynames :: variant :: Resource = self . cldr () ? . displaynames () . read_and_parse (req . id . locale , "variants.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (VariantDisplayNames :: try_from (data) . map_err (| e | { DataError :: custom ("data for VariantDisplayNames") . with_display_context (& e) }) ?) , }) } }
};
}
