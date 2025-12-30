// Generated macro for impl_495 (impl)
macro_rules! Depcrate_currency_extendedimpl_495 {
() => {
// Module: crate::currency::extended
// Provides: {"impl_495"}
// Dependencies: {}
impl DataProvider < CurrencyExtendedDataV1 > for crate :: SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CurrencyExtendedDataV1 > , DataError > { self . check_req :: < CurrencyExtendedDataV1 > (req) ? ; let currencies_resource : & cldr_serde :: currencies :: data :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "currencies.json") ? ; let currency = currencies_resource . main . value . numbers . currencies . get (req . id . marker_attributes . as_str ()) . ok_or (DataError :: custom ("No currency associated with the aux key")) ? ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (CurrencyExtendedData { display_names : PluralElements :: new (currency . other . as_deref () . ok_or_else (| | DataErrorKind :: IdentifierNotFound . into_error ()) ? ,) . with_zero_value (currency . zero . as_deref ()) . with_one_value (currency . one . as_deref ()) . with_two_value (currency . two . as_deref ()) . with_few_value (currency . few . as_deref ()) . with_many_value (currency . many . as_deref ()) . with_explicit_one_value (currency . explicit_one . as_deref ()) . with_explicit_zero_value (currency . explicit_zero . as_deref ()) . into () , }) , }) } }
};
}
