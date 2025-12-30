// Generated macro for borrowed_internal_visit (function)
macro_rules! Depcrate_internal_sval_v2borrowed_internal_visit {
() => {
// Module: crate::internal::sval::v2
// Provides: {"borrowed_internal_visit"}
// Dependencies: {}
pub (crate) fn borrowed_internal_visit < 'v > (v : & 'v dyn Value , visitor : & mut dyn InternalVisitor < 'v > ,) -> bool { let mut visitor = VisitorStream { visitor , text_buf : Default :: default () , } ; value_bag_sval2 :: lib :: stream (& mut visitor , v) . is_ok () }
};
}
