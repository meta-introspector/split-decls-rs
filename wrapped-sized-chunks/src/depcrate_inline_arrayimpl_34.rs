// Generated macro for impl_34 (impl)
macro_rules! Depcrate_inline_arrayimpl_34 {
() => {
// Module: crate::inline_array
// Provides: {"impl_34"}
// Dependencies: {}
impl < A , T > DerefMut for InlineArray < A , T > { fn deref_mut (& mut self) -> & mut Self :: Target { unsafe { from_raw_parts_mut (self . data_mut () , self . len ()) } } }
};
}
