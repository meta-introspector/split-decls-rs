// Generated macro for check_static_nonnull (function)
macro_rules! Depcratecheck_static_nonnull {
() => {
// Module: crate
// Provides: {"check_static_nonnull"}
// Dependencies: {}
# [track_caller] pub fn check_static_nonnull < T : ? Sized > (var : & T) { let ptr : * const T = var ; if ptr . is_null () { panic ! ("static was marked as NonNull, so it must not be NULL") ; } }
};
}
