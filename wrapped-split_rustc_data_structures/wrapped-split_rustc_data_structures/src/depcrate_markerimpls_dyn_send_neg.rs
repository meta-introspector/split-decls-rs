// Generated macro for impls_dyn_send_neg (macro)
macro_rules! Depcrate_markerimpls_dyn_send_neg {
() => {
// Module: crate::marker
// Provides: {"impls_dyn_send_neg"}
// Dependencies: {}
macro_rules ! impls_dyn_send_neg { ($ ([$ t1 : ty $ (where $ ($ generics1 : tt) *) ?]) *) => { $ (impl $ (<$ ($ generics1) *>) ? ! DynSend for $ t1 { }) * } ; }
};
}
