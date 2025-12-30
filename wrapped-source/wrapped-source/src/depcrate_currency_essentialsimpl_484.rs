// Generated macro for impl_484 (impl)
macro_rules! Depcrate_currency_essentialsimpl_484 {
() => {
// Module: crate::currency::essentials
// Provides: {"impl_484"}
// Dependencies: {}
impl DataProvider < CurrencyEssentialsV1 > for SourceDataProvider { fn load (& self , req : DataRequest) -> Result < DataResponse < CurrencyEssentialsV1 > , DataError > { self . check_req :: < CurrencyEssentialsV1 > (req) ? ; let currencies_resource : & cldr_serde :: currencies :: data :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "currencies.json") ? ; let numbers_resource : & cldr_serde :: numbers :: Resource = self . cldr () ? . numbers () . read_and_parse (req . id . locale , "numbers.json") ? ; let result = extract_currency_essentials (self , currencies_resource , numbers_resource) ; Ok (DataResponse { metadata : Default :: default () , payload : DataPayload :: from_owned (result ?) , }) } }
};
}
