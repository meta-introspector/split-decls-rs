// Generated macro for into_non_null (function)
macro_rules! Depcrate_hash_tableinto_non_null {
() => {
// Module: crate::hash_table
// Provides: {"into_non_null"}
// Dependencies: {}
# [doc = " Turns a reference into a [`NonNull`] pointer."] const fn into_non_null < T : Sized > (t : & T) -> NonNull < T > { unsafe { NonNull :: new_unchecked (from_ref (t) . cast_mut ()) } }
};
}
