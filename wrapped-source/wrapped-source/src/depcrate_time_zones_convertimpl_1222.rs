// Generated macro for impl_1222 (impl)
macro_rules! Depcrate_time_zones_convertimpl_1222 {
() => {
// Module: crate::time_zones::convert
// Provides: {"impl_1222"}
// Dependencies: {}
impl DataProvider < TimezoneNamesCitiesOverrideV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < TimezoneNamesCitiesOverrideV1 > , DataError > { self . check_req :: < TimezoneNamesCitiesOverrideV1 > (req) ? ; let mut exemplars = self . calculate_locations (req . id . locale) ? . 1 ; let base = DataProvider :: < TimezoneNamesCitiesRootV1 > :: load (& self , req) ? . payload ; exemplars . retain (| k , v | base . get () . exemplars . get (k) != Some (v)) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (ExemplarCities { exemplars : exemplars . into_iter () . collect () , }) , }) } }
};
}
