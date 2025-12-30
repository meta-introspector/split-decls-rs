// Generated macro for io_uring_getevents_arg (struct)
macro_rules! Depcrate_io_uringio_uring_getevents_arg {
() => {
// Module: crate::io_uring
// Provides: {"io_uring_getevents_arg"}
// Dependencies: {}
# [allow (missing_docs)] # [repr (C)] # [derive (Debug , Copy , Clone , Default)] pub struct io_uring_getevents_arg { pub sigmask : io_uring_ptr , pub sigmask_sz : u32 , pub min_wait_usec : u32 , pub ts : io_uring_ptr , }
};
}
