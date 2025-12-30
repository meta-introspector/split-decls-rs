// Generated macro for impl_967 (impl)
macro_rules! Depcrate_percentimpl_967 {
() => {
// Module: crate::percent
// Provides: {"impl_967"}
// Dependencies: {}
impl DataProvider < PercentEssentialsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < PercentEssentialsV1 > , DataError > { self . check_req :: < PercentEssentialsV1 > (req) ? ; let numbers_resource : & cldr_serde :: numbers :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "numbers.json") ? ; let result = extract_percent_essentials (numbers_resource) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (result ?) , }) } }
};
}
