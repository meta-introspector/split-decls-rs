// Generated macro for impl_457 (impl)
macro_rules! Depcrate_currency_displaynameimpl_457 {
() => {
// Module: crate::currency::displayname
// Provides: {"impl_457"}
// Dependencies: {}
impl DataProvider < CurrencyDisplaynameV1 > for crate :: SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CurrencyDisplaynameV1 > , DataError > { self . check_req :: < CurrencyDisplaynameV1 > (req) ? ; let currencies_resource : & cldr_serde :: currencies :: data :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "currencies.json") ? ; let currency = currencies_resource . main . value . numbers . currencies . get (req . id . marker_attributes . as_str ()) . ok_or_else (| | { DataErrorKind :: IdentifierNotFound . into_error () . with_debug_context ("No data for currency") }) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (CurrencyDisplayname { display_name : Cow :: Owned (currency . display_name . as_deref () . ok_or_else (| | { DataErrorKind :: IdentifierNotFound . into_error () . with_debug_context ("No display name found for the currency") }) ? . to_string () ,) , }) , }) } }
};
}
