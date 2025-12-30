// Generated macro for visit_counts (function)
macro_rules! Depcrate_visit_countvisit_counts {
() => {
// Module: crate::visit_count
// Provides: {"visit_counts"}
// Dependencies: {}
pub fn visit_counts (val : & impl Valuable) -> VisitCount { let mut visit = VisitCount :: default () ; val . visit (& mut visit) ; visit }
};
}
