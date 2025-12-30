// Generated macro for impl_496 (impl)
macro_rules! Depcrate_currency_extendedimpl_496 {
() => {
// Module: crate::currency::extended
// Provides: {"impl_496"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < CurrencyExtendedDataV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { let mut result = HashSet :: new () ; let numbers = self . cldr () ? . numbers () ; let locales = numbers . list_locales () ? ; for locale in locales { let currencies_resource : & cldr_serde :: currencies :: data :: Resource = self . cldr () ? . numbers () . read_and_parse (& locale , "currencies.json") ? ; let currencies = & currencies_resource . main . value . numbers . currencies ; for (currency , displaynames) in currencies { if displaynames . other . is_none () { continue ; } if let Ok (attributes) = DataMarkerAttributes :: try_from_string (currency . clone ()) { result . insert (DataIdentifierCow :: from_owned (attributes , locale)) ; } } } Ok (result) } }
};
}
