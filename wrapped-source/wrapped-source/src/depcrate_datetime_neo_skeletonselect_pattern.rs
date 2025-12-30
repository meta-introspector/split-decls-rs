// Generated macro for select_pattern (function)
macro_rules! Depcrate_datetime_neo_skeletonselect_pattern {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"select_pattern"}
// Dependencies: {}
fn select_pattern < 'data > (bag : components :: Bag , skeletons : & BTreeMap < Skeleton , PluralElements < Pattern < 'data > > > , preferred_hour_cycle : CoarseHourCycle , length_patterns : & GenericLengthPatterns < 'data > ,) -> PatternsWithDistance < PluralElements < Pattern < 'data > > > { use icu :: datetime :: provider :: pattern :: { runtime , PatternItem } ; use icu_locale_core :: preferences :: extensions :: unicode :: keywords :: HourCycle ; let default_hour_cycle = match preferred_hour_cycle { CoarseHourCycle :: H11H12 => HourCycle :: H12 , CoarseHourCycle :: H23 => HourCycle :: H23 , } ; let fields = bag . to_vec_fields (default_hour_cycle) ; match create_best_pattern_for_fields (skeletons , length_patterns , & fields , & bag , false) { BestSkeleton :: AllFieldsMatch (p , distance) => PatternsWithDistance { inner : p , distance } , BestSkeleton :: MissingOrExtraFields (p , distance) => { PatternsWithDistance { inner : p , distance } } BestSkeleton :: NoMatch => { let pattern_items = fields . into_iter () . flat_map (| field | [PatternItem :: Literal (' ') , PatternItem :: Field (field)]) . skip (1) . collect :: < Vec < _ > > () ; let pattern = runtime :: Pattern :: from (pattern_items) ; PatternsWithDistance { inner : PluralElements :: new (pattern) , distance : SkeletonQuality :: worst () , } } } }
};
}
