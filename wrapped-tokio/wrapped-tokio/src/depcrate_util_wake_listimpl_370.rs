// Generated macro for impl_370 (impl)
macro_rules! Depcrate_util_wake_listimpl_370 {
() => {
// Module: crate::util::wake_list
// Provides: {"impl_370"}
// Dependencies: {}
impl Drop for WakeList { fn drop (& mut self) { let slice = ptr :: slice_from_raw_parts_mut (self . inner . as_mut_ptr () . cast :: < Waker > () , self . curr) ; unsafe { ptr :: drop_in_place (slice) } ; } }
};
}
