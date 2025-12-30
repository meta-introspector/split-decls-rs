// Generated macro for internal_visit (function)
macro_rules! Depcrate_internal_sval_v2internal_visit {
() => {
// Module: crate::internal::sval::v2
// Provides: {"internal_visit"}
// Dependencies: {}
pub (crate) fn internal_visit (v : & dyn Value , visitor : & mut dyn InternalVisitor < '_ >) -> bool { let mut visitor = VisitorStream { visitor , text_buf : Default :: default () , } ; value_bag_sval2 :: lib :: stream_computed (& mut visitor , v) . is_ok () }
};
}
