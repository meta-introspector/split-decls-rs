// Generated macro for impl_1008 (impl)
macro_rules! Depcrate_pluralsimpl_1008 {
() => {
// Module: crate::plurals
// Provides: {"impl_1008"}
// Dependencies: {}
# [cfg (feature = "experimental")] impl From < & cldr_serde :: plural_ranges :: LocalePluralRanges > for PluralRanges < 'static > { fn from (other : & cldr_serde :: plural_ranges :: LocalePluralRanges) -> Self { fn convert (s : & str) -> RawPluralCategory { PluralCategory :: get_for_cldr_string (s) . expect ("category parsing failed.") . into () } let mut map : BTreeMap < (RawPluralCategory , RawPluralCategory) , RawPluralCategory > = BTreeMap :: new () ; for (range , result) in & other . 0 { let start = convert (& range . start) ; let end = convert (& range . end) ; let result = convert (result) ; if end != result { map . insert ((start , end) , result) ; } } PluralRanges { ranges : map . into_iter () . map (| ((start , end) , result) | { (UnvalidatedPluralRange :: from_range (start , end) , result) }) . collect () , } } }
};
}
