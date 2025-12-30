// Generated macro for impl_33 (impl)
macro_rules! Depcrate_inline_arrayimpl_33 {
() => {
// Module: crate::inline_array
// Provides: {"impl_33"}
// Dependencies: {}
impl < A , T > Deref for InlineArray < A , T > { type Target = [A] ; fn deref (& self) -> & Self :: Target { unsafe { from_raw_parts (self . data () , self . len ()) } } }
};
}
