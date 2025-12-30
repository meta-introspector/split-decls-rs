// Generated macro for tests (module)
macro_rules! Depcrate_event_selecttests {
() => {
// Module: crate::event::select
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use core :: mem :: { align_of , size_of } ; # [test] # [cfg (any (windows , target_os = "wasi"))] fn layouts () { assert_eq ! (align_of ::< FdSetElement > () , align_of ::< FD_SET > ()) ; assert_eq ! (fd_set_num_elements_for_fd_array_raw (memoffset :: span_of ! (FD_SET , fd_array) . len () / size_of ::< RawFd > ()) * size_of ::< FdSetElement > () , size_of ::< FD_SET > ()) ; assert_eq ! (fd_set_num_elements_for_fd_array (memoffset :: span_of ! (FD_SET , fd_array) . len () / size_of ::< RawFd > ()) * size_of ::< FdSetElement > () , size_of ::< FD_SET > ()) ; assert_eq ! (fd_set_num_elements_for_fd_array (0) * size_of ::< FdSetElement > () , size_of ::< FD_SET > ()) ; } # [test] # [cfg (any (bsd , linux_kernel))] fn layouts () { use crate :: backend :: c ; assert_eq ! (align_of ::< FdSetElement > () , align_of ::< c :: fd_set > ()) ; assert_eq ! (fd_set_num_elements_for_bitvector (c :: FD_SETSIZE as RawFd) * size_of ::< FdSetElement > () , size_of ::< c :: fd_set > ()) ; } }
};
}
