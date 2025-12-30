// Generated macro for pin_as_deref_mut (function)
macro_rules! Depcrate_utilpin_as_deref_mut {
() => {
// Module: crate::util
// Provides: {"pin_as_deref_mut"}
// Dependencies: {}
# [doc = " Copy of [`std::pin::Pin::as_deref_mut`]."] pub (crate) fn pin_as_deref_mut < P : DerefMut > (ptr : Pin < & mut Pin < P > >) -> Pin < & mut P :: Target > { unsafe { ptr . get_unchecked_mut () } . as_mut () }
};
}
