// Generated macro for impl_1345 (impl)
macro_rules! Depcrate_units_display_namesimpl_1345 {
() => {
// Module: crate::units::display_names
// Provides: {"impl_1345"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < UnitsDisplayNamesV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { let mut data_locales = HashSet :: new () ; let numbers = self . cldr () ? . numbers () ; let locales = numbers . list_locales () ? ; for locale in locales { let units_format_data : & cldr_serde :: units :: data :: Resource = self . cldr () ? . units () . read_and_parse (& locale , "units.json") ? ; let units_format_data = & units_format_data . main . value . units ; for length in & ["long" , "short" , "narrow"] { let length_patterns = match * length { "long" => & units_format_data . long , "short" => & units_format_data . short , "narrow" => & units_format_data . narrow , _ => { return Err (DataErrorKind :: IdentifierNotFound . into_error () . with_debug_context (length)) } } ; for units_map in length_patterns . categories . values () { for (unit , patterns) in units_map { if patterns . other . is_none () { continue ; } data_locales . insert (DataIdentifierCow :: from_owned (DataMarkerAttributes :: try_from_string (format ! ("{length}-{unit}")) . map_err (| _ | { DataError :: custom ("Failed to parse the attribute") . with_debug_context (& unit) }) ? , locale ,)) ; } } } } Ok (data_locales) } }
};
}
