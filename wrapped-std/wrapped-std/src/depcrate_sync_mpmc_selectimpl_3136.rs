// Generated macro for impl_3136 (impl)
macro_rules! Depcrate_sync_mpmc_selectimpl_3136 {
() => {
// Module: crate::sync::mpmc::select
// Provides: {"impl_3136"}
// Dependencies: {}
impl Operation { # [doc = " Creates an operation identifier from a mutable reference."] # [doc = ""] # [doc = " This function essentially just turns the address of the reference into a number. The"] # [doc = " reference should point to a variable that is specific to the thread and the operation,"] # [doc = " and is alive for the entire duration of a blocking operation."] # [inline] pub fn hook < T > (r : & mut T) -> Operation { let val = r as * mut T as usize ; assert ! (val > 2) ; Operation (val) } }
};
}
