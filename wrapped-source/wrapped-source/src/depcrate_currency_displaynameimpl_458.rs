// Generated macro for impl_458 (impl)
macro_rules! Depcrate_currency_displaynameimpl_458 {
() => {
// Module: crate::currency::displayname
// Provides: {"impl_458"}
// Dependencies: {}
impl crate :: IterableDataProviderCached < CurrencyDisplaynameV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { let mut result = HashSet :: new () ; let numbers = self . cldr () ? . numbers () ; let locales = numbers . list_locales () ? ; for locale in locales { let currencies_resource : & cldr_serde :: currencies :: data :: Resource = self . cldr () ? . numbers () . read_and_parse (& locale , "currencies.json") ? ; let currencies = & currencies_resource . main . value . numbers . currencies ; for (currency , patterns) in currencies { if patterns . display_name . is_none () { continue ; } if let Ok (attributes) = DataMarkerAttributes :: try_from_string (currency . clone ()) { result . insert (DataIdentifierCow :: from_owned (attributes , locale)) ; } } } Ok (result) } }
};
}
