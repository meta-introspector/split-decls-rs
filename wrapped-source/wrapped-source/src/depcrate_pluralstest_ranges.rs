// Generated macro for test_ranges (function)
macro_rules! Depcrate_pluralstest_ranges {
() => {
// Module: crate::plurals
// Provides: {"test_ranges"}
// Dependencies: {}
# [test] # [cfg (feature = "experimental")] fn test_ranges () { use icu :: locale :: langid ; let provider = SourceDataProvider :: new_testing () ; let plural_ranges : DataResponse < PluralsRangesV1 > = provider . load (DataRequest { id : DataIdentifierCow :: from_locale (langid ! ("sl") . into ()) . as_borrowed () , .. Default :: default () }) . unwrap () ; assert_eq ! (plural_ranges . payload . get () . ranges . get_copied (& UnvalidatedPluralRange :: from_range (RawPluralCategory :: Few , RawPluralCategory :: One)) , Some (RawPluralCategory :: Few)) ; assert_eq ! (plural_ranges . payload . get () . ranges . get_copied (& UnvalidatedPluralRange :: from_range (RawPluralCategory :: Other , RawPluralCategory :: One)) , Some (RawPluralCategory :: Few)) ; assert ! (plural_ranges . payload . get () . ranges . get_copied (& UnvalidatedPluralRange :: from_range (RawPluralCategory :: Zero , RawPluralCategory :: One)) . is_none ()) ; assert ! (plural_ranges . payload . get () . ranges . get_copied (& UnvalidatedPluralRange :: from_range (RawPluralCategory :: One , RawPluralCategory :: Zero)) . is_none ()) ; assert ! (plural_ranges . payload . get () . ranges . get_copied (& UnvalidatedPluralRange :: from_range (RawPluralCategory :: One , RawPluralCategory :: Other)) . is_none ()) ; assert ! (plural_ranges . payload . get () . ranges . get_copied (& UnvalidatedPluralRange :: from_range (RawPluralCategory :: Few , RawPluralCategory :: Two)) . is_none ()) ; }
};
}
