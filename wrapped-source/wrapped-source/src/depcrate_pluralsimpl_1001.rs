// Generated macro for impl_1001 (impl)
macro_rules! Depcrate_pluralsimpl_1001 {
() => {
// Module: crate::plurals
// Provides: {"impl_1001"}
// Dependencies: {}
impl SourceDataProvider { fn get_rules_for (& self , marker : DataMarkerInfo ,) -> Result < & cldr_serde :: plurals :: Rules , DataError > { if marker == PluralsCardinalV1 :: INFO { self . cldr () ? . core () . read_and_parse :: < cldr_serde :: plurals :: Resource > ("supplemental/plurals.json") ? . supplemental . plurals_type_cardinal . as_ref () } else if marker == PluralsOrdinalV1 :: INFO { self . cldr () ? . core () . read_and_parse :: < cldr_serde :: plurals :: Resource > ("supplemental/ordinals.json") ? . supplemental . plurals_type_ordinal . as_ref () } else { None } . ok_or (DataError :: custom ("Unknown marker for PluralRules")) } # [cfg (feature = "experimental")] fn get_plural_ranges (& self) -> Result < & cldr_serde :: plural_ranges :: PluralRanges , DataError > { Ok (& self . cldr () ? . core () . read_and_parse :: < cldr_serde :: plural_ranges :: Resource > ("supplemental/pluralRanges.json" ,) ? . supplemental . plurals) } }
};
}
