// Generated macro for impl_3749 (impl)
macro_rules! Depcrate_sys_common_wstrimpl_3749 {
() => {
// Module: crate::sys_common::wstr
// Provides: {"impl_3749"}
// Dependencies: {}
impl Iterator for WStrUnits < '_ > { type Item = NonZero < u16 > ; fn next (& mut self) -> Option < Self :: Item > { unsafe { let next = self . peek () ? ; self . lpwstr = NonNull :: new_unchecked (self . lpwstr . as_ptr () . add (1)) ; Some (next) } } }
};
}
