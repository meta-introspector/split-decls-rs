// Generated macro for impl_799 (impl)
macro_rules! Depcrate_displaynames_regionimpl_799 {
() => {
// Module: crate::displaynames::region
// Provides: {"impl_799"}
// Dependencies: {}
impl DataProvider < RegionDisplayNamesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < RegionDisplayNamesV1 > , DataError > { self . check_req :: < RegionDisplayNamesV1 > (req) ? ; let data : & cldr_serde :: displaynames :: region :: Resource = self . cldr () ? . displaynames () . read_and_parse (req . id . locale , "territories.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (RegionDisplayNames :: try_from (data) . map_err (| e | { DataError :: custom ("data for RegionDisplayNames") . with_display_context (& e) }) ?) , }) } }
};
}
