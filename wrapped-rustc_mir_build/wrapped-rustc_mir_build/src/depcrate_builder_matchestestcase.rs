// Generated macro for TestCase (enum)
macro_rules! Depcrate_builder_matchesTestCase {
() => {
// Module: crate::builder::matches
// Provides: {"TestCase"}
// Dependencies: {}
# [doc = " Partial summary of a [`thir::Pat`], indicating what sort of test should be"] # [doc = " performed to match/reject the pattern, and what the desired test outcome is."] # [doc = " This avoids having to perform a full match on [`thir::PatKind`] in some places,"] # [doc = " and helps [`TestKind::Switch`] and [`TestKind::SwitchInt`] know what target"] # [doc = " values to use."] # [doc = ""] # [doc = " Created by [`MatchPairTree::for_pattern`], and then inspected primarily by:"] # [doc = " - [`Builder::pick_test_for_match_pair`] (to choose a test)"] # [doc = " - [`Builder::sort_candidate`] (to see how the test interacts with a match pair)"] # [doc = ""] # [doc = " Note that or-patterns are not tested directly like the other variants."] # [doc = " Instead they participate in or-pattern expansion, where they are transformed into"] # [doc = " subcandidates. See [`Builder::expand_and_match_or_candidates`]."] # [derive (Debug , Clone)] enum TestCase < 'tcx > { Variant { adt_def : ty :: AdtDef < 'tcx > , variant_index : VariantIdx } , Constant { value : ty :: Value < 'tcx > } , Range (Arc < PatRange < 'tcx > >) , Slice { len : usize , variable_length : bool } , Deref { temp : Place < 'tcx > , mutability : Mutability } , Never , Or { pats : Box < [FlatPat < 'tcx >] > } , }
};
}
