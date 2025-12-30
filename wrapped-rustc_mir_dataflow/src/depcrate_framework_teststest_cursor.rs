// Generated macro for test_cursor (function)
macro_rules! Depcrate_framework_teststest_cursor {
() => {
// Module: crate::framework::tests
// Provides: {"test_cursor"}
// Dependencies: {}
fn test_cursor < D : Direction > (analysis : MockAnalysis < '_ , D >) { let body = analysis . body ; let mut cursor = AnalysisAndResults { results : analysis . mock_results () , analysis } . into_results_cursor (body) ; cursor . allow_unreachable () ; let every_target = | | { body . basic_blocks . iter_enumerated () . flat_map (| (bb , _) | SeekTarget :: iter_in_block (body , bb)) } ; let mut seek_to_target = | targ | { use SeekTarget :: * ; match targ { BlockEntry (block) => cursor . seek_to_block_entry (block) , Early (loc) => cursor . seek_before_primary_effect (loc) , After (loc) => cursor . seek_after_primary_effect (loc) , } assert_eq ! (cursor . get () , & cursor . analysis () . expected_state_at_target (targ)) ; } ; for from in every_target () { seek_to_target (from) ; for to in every_target () { dbg ! (from) ; dbg ! (to) ; seek_to_target (to) ; seek_to_target (from) ; } } }
};
}
