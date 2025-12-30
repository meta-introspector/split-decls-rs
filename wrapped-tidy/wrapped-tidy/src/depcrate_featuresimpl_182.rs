// Generated macro for impl_182 (impl)
macro_rules! Depcrate_featuresimpl_182 {
() => {
// Module: crate::features
// Provides: {"impl_182"}
// Dependencies: {}
impl Feature { fn tracking_issue_display (& self) -> impl fmt :: Display { match self . tracking_issue { None => "none" . to_string () , Some (x) => x . to_string () , } } }
};
}
