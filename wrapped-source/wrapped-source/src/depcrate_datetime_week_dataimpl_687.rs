// Generated macro for impl_687 (impl)
macro_rules! Depcrate_datetime_week_dataimpl_687 {
() => {
// Module: crate::datetime::week_data
// Provides: {"impl_687"}
// Dependencies: {}
impl IterableDataProviderCached < CalendarWeekV1 > for SourceDataProvider { fn iter_ids_cached (& self) -> Result < HashSet < DataIdentifierCow < 'static > > , DataError > { let week_data : & cldr_serde :: week_data :: Resource = self . cldr () ? . core () . read_and_parse ("supplemental/weekData.json") ? ; let week_data = & week_data . supplemental . week_data ; Ok (week_data . min_days . keys () . chain (week_data . first_day . keys ()) . chain (week_data . weekend_end . keys ()) . chain (week_data . weekend_start . keys ()) . filter_map (| t | match t { & DEFAULT_TERRITORY => Some (None) , Territory :: Region (r) => Some (Some (* r)) , _ => None , }) . map (| region | { let mut locale = DataLocale :: default () ; locale . region = region ; DataIdentifierCow :: from_locale (locale) }) . collect ()) } }
};
}
