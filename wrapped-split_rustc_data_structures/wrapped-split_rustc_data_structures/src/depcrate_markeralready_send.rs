// Generated macro for already_send (macro)
macro_rules! Depcrate_markeralready_send {
() => {
// Module: crate::marker
// Provides: {"already_send"}
// Dependencies: {}
macro_rules ! already_send { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSend for $ ty where $ ty : Send { }) * } ; }
};
}
