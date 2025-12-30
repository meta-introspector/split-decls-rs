// Generated macro for check (function)
macro_rules! Depcrate_tests_placementcheck {
() => {
// Module: crate::tests_placement
// Provides: {"check"}
// Dependencies: {}
pub fn check (root_path : impl AsRef < Path > , bad : & mut bool) { if root_path . as_ref () . join (FORBIDDEN_PATH) . exists () { tidy_error ! (bad , "Tests have been moved, please move them from {} to {}" , root_path . as_ref () . join (FORBIDDEN_PATH) . display () , root_path . as_ref () . join (ALLOWED_PATH) . display ()) } }
};
}
