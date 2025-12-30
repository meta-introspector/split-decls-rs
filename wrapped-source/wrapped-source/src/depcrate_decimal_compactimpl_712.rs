// Generated macro for impl_712 (impl)
macro_rules! Depcrate_decimal_compactimpl_712 {
() => {
// Module: crate::decimal::compact
// Provides: {"impl_712"}
// Dependencies: {}
impl DataProvider < ShortCompactDecimalFormatDataV1 > for SourceDataProvider { fn load (& self , req : DataRequest ,) -> Result < DataResponse < ShortCompactDecimalFormatDataV1 > , DataError > { self . check_req :: < ShortCompactDecimalFormatDataV1 > (req) ? ; let resource : & cldr_serde :: numbers :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "numbers.json") ? ; let numbers = & resource . main . value . numbers ; let nsname = if ! req . id . marker_attributes . is_empty () { req . id . marker_attributes . as_str () } else { & numbers . default_numbering_system } ; let result = CompactDecimalPatternData :: try_from (& numbers . numsys_data . formats . get (nsname) . ok_or_else (| | { DataError :: custom ("Could not find formats for numbering system") . with_display_context (nsname) }) ? . short . decimal_format ,) . map_err (| s | { DataError :: custom ("Could not create compact decimal patterns") . with_display_context (& s) . with_display_context (nsname) }) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (result) , }) } }
};
}
