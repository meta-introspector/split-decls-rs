// Generated macro for impl_1223 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1223 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1223"}
// Dependencies: {}
impl DataProvider < TimezoneNamesCitiesRootV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < TimezoneNamesCitiesRootV1 > , DataError > { self . check_req :: < TimezoneNamesCitiesRootV1 > (req) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (ExemplarCities { exemplars : self . calculate_locations (& self . dedupe_group (* req . id . locale) ?) ? . 1 . into_iter () . collect () , }) , }) } }
};
}
