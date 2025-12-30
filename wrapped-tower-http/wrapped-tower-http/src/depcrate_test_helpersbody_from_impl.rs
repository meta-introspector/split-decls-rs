// Generated macro for body_from_impl (macro)
macro_rules! Depcrate_test_helpersbody_from_impl {
() => {
// Module: crate::test_helpers
// Provides: {"body_from_impl"}
// Dependencies: {}
macro_rules ! body_from_impl { ($ ty : ty) => { impl From <$ ty > for Body { fn from (buf : $ ty) -> Self { Self :: new (http_body_util :: Full :: from (buf)) } } } ; }
};
}
