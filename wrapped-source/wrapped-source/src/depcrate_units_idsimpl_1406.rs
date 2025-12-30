// Generated macro for impl_1406 (impl)
macro_rules! Depcrate_units_idsimpl_1406 {
() => {
// Module: crate::units::ids
// Provides: {"impl_1406"}
// Dependencies: {}
impl DataProvider < UnitIdsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < UnitIdsV1 > , DataError > { self . check_req :: < UnitIdsV1 > (req) ? ; let units_data : & cldr_serde :: units :: info :: Resource = self . cldr () ? . core () . read_and_parse ("supplemental/units.json") ? ; let unit = req . id . marker_attributes . as_str () ; Ok (DataResponse { payload : DataPayload :: from_owned (units_data . unit_id (unit) ?) , metadata : Default :: default () , }) } }
};
}
