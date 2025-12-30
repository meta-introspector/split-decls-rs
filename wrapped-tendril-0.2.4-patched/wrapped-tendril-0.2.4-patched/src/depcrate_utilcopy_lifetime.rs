// Generated macro for copy_lifetime (function)
macro_rules! Depcrate_utilcopy_lifetime {
() => {
// Module: crate::util
// Provides: {"copy_lifetime"}
// Dependencies: {}
# [inline (always)] pub unsafe fn copy_lifetime < 'a , S : ? Sized , T : ? Sized + 'a > (_ptr : & 'a S , ptr : & T) -> & 'a T { mem :: transmute (ptr) }
};
}
