// Generated macro for visit_reachable_results (function)
macro_rules! Depcrate_framework_visitorvisit_reachable_results {
() => {
// Module: crate::framework::visitor
// Provides: {"visit_reachable_results"}
// Dependencies: {}
# [doc = " Like `visit_results`, but only for reachable blocks."] pub fn visit_reachable_results < 'mir , 'tcx , A > (body : & 'mir mir :: Body < 'tcx > , analysis : & mut A , results : & Results < A :: Domain > , vis : & mut impl ResultsVisitor < 'tcx , A > ,) where A : Analysis < 'tcx > , { let blocks = traversal :: reachable (body) . map (| (bb , _) | bb) ; visit_results (body , blocks , analysis , results , vis) }
};
}
