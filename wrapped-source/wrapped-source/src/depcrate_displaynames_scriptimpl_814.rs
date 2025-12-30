// Generated macro for impl_814 (impl)
macro_rules! Depcrate_displaynames_scriptimpl_814 {
() => {
// Module: crate::displaynames::script
// Provides: {"impl_814"}
// Dependencies: {}
impl DataProvider < ScriptDisplayNamesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < ScriptDisplayNamesV1 > , DataError > { self . check_req :: < ScriptDisplayNamesV1 > (req) ? ; let data : & cldr_serde :: displaynames :: script :: Resource = self . cldr () ? . displaynames () . read_and_parse (req . id . locale , "scripts.json") ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (ScriptDisplayNames :: try_from (data) . map_err (| e | { DataError :: custom ("data for ScriptDisplayNames") . with_display_context (& e) }) ?) , }) } }
};
}
