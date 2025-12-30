// Generated macro for impl_755 (impl)
macro_rules! Depcrate_decimal_symbolsimpl_755 {
() => {
// Module: crate::decimal::symbols
// Provides: {"impl_755"}
// Dependencies: {}
impl DataProvider < DecimalSymbolsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < DecimalSymbolsV1 > , DataError > { self . check_req :: < DecimalSymbolsV1 > (req) ? ; let resource : & cldr_serde :: numbers :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "numbers.json") ? ; let numbers = & resource . main . value . numbers ; let nsname = if ! req . id . marker_attributes . is_empty () { req . id . marker_attributes . as_str () } else { & numbers . default_numbering_system } ; let result = DecimalSymbols :: try_from (NumbersWithNumsys (numbers , nsname)) . map_err (| s | { DataError :: custom ("Could not create decimal symbols") . with_display_context (& s) . with_display_context (nsname) }) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (result) , }) } }
};
}
