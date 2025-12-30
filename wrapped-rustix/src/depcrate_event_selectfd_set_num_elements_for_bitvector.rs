// Generated macro for fd_set_num_elements_for_bitvector (function)
macro_rules! Depcrate_event_selectfd_set_num_elements_for_bitvector {
() => {
// Module: crate::event::select
// Provides: {"fd_set_num_elements_for_bitvector"}
// Dependencies: {}
# [doc = " `fd_set_num_elements` implementation on platforms with bitvector"] # [doc = " implementations."] # [cfg (not (any (windows , target_os = "wasi")))] # [inline] pub (crate) fn fd_set_num_elements_for_bitvector (nfds : RawFd) -> usize { let nfds = nfds as usize ; div_ceil (nfds , BITS) }
};
}
