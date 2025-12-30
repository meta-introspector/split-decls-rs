// Generated macro for copy_lifetime_mut (function)
macro_rules! Depcrate_utilcopy_lifetime_mut {
() => {
// Module: crate::util
// Provides: {"copy_lifetime_mut"}
// Dependencies: {}
# [inline (always)] pub unsafe fn copy_lifetime_mut < 'a , S : ? Sized , T : ? Sized + 'a > (_ptr : & 'a mut S , ptr : & mut T) -> & 'a mut T { mem :: transmute (ptr) }
};
}
