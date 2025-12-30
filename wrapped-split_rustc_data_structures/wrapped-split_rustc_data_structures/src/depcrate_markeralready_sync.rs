// Generated macro for already_sync (macro)
macro_rules! Depcrate_markeralready_sync {
() => {
// Module: crate::marker
// Provides: {"already_sync"}
// Dependencies: {}
macro_rules ! already_sync { ($ ([$ ty : ty]) *) => { $ (unsafe impl DynSync for $ ty where $ ty : Sync { }) * } ; }
};
}
