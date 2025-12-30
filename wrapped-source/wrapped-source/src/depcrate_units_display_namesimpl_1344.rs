// Generated macro for impl_1344 (impl)
macro_rules! Depcrate_units_display_namesimpl_1344 {
() => {
// Module: crate::units::display_names
// Provides: {"impl_1344"}
// Dependencies: {}
impl DataProvider < UnitsDisplayNamesV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < UnitsDisplayNamesV1 > , DataError > { self . check_req :: < UnitsDisplayNamesV1 > (req) ? ; let (length , unit) = req . id . marker_attributes . split_once ('-') . ok_or_else (| | { DataErrorKind :: InvalidRequest . into_error () . with_req (UnitsDisplayNamesV1 :: INFO , req) }) ? ; let units_format_data : & cldr_serde :: units :: data :: Resource = self . cldr () ? . units () . read_and_parse (req . id . locale , "units.json") ? ; let units_format_data = & units_format_data . main . value . units ; let unit_patterns = match length { "long" => & units_format_data . long , "short" => & units_format_data . short , "narrow" => & units_format_data . narrow , _ => { return Err (DataErrorKind :: InvalidRequest . into_error () . with_debug_context (length)) } } . categories . iter () . find_map (| (_ , units_map) | units_map . get (unit)) . ok_or_else (| | { DataErrorKind :: IdentifierNotFound . into_error () . with_debug_context (length) }) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (UnitsDisplayNames { patterns : unit_patterns . try_into_plural_elements_packed_cow () ? , }) , }) } }
};
}
