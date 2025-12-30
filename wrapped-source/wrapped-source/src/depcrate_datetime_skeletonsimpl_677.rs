// Generated macro for impl_677 (impl)
macro_rules! Depcrate_datetime_skeletonsimpl_677 {
() => {
// Module: crate::datetime::skeletons
// Provides: {"impl_677"}
// Dependencies: {}
impl cldr_serde :: ca :: AvailableFormats { pub fn parse_skeletons (& self) -> BTreeMap < Skeleton , PluralElements < Pattern < 'static > > > { let mut patterns : BTreeMap < String , BTreeMap < PluralCategory , String > > = BTreeMap :: new () ; for (skeleton_str , pattern_str) in self . 0 . iter () { let (skeleton , plural_category) = match skeleton_str . split_once ("-count-") { Some ((s , v)) => (s , PluralCategory :: get_for_cldr_string (v) . unwrap ()) , None => (skeleton_str . as_ref () , PluralCategory :: Other) , } ; patterns . entry (skeleton . to_string ()) . or_default () . insert (plural_category , pattern_str . to_string ()) ; } let skeletons = patterns . iter () . filter_map (| (skeleton_str , patterns) | { let skeleton = match Skeleton :: try_from (skeleton_str . as_str ()) { Ok (s) => s , Err (SkeletonError :: SymbolUnimplemented (_)) => return None , Err (SkeletonError :: SkeletonHasVariant) => return None , Err (err) => panic ! ("Unexpected skeleton error while parsing skeleton {skeleton_str:?} {err}") , } ; let patterns = PluralElements :: new (& patterns [& PluralCategory :: Other]) . with_zero_value (patterns . get (& PluralCategory :: Zero)) . with_one_value (patterns . get (& PluralCategory :: One)) . with_two_value (patterns . get (& PluralCategory :: Two)) . with_few_value (patterns . get (& PluralCategory :: Few)) . with_many_value (patterns . get (& PluralCategory :: Many)) . map (| s | s . parse () . expect (s)) ; Some ((skeleton , patterns)) }) . collect () ; skeletons } }
};
}
