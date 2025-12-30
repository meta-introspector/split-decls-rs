// Generated macro for OUTPUT_CAPTURE_USED (static)
macro_rules! Depcrate_io_stdioOUTPUT_CAPTURE_USED {
() => {
// Module: crate::io::stdio
// Provides: {"OUTPUT_CAPTURE_USED"}
// Dependencies: {}
# [doc = " Flag to indicate OUTPUT_CAPTURE is used."] # [doc = ""] # [doc = " If it is None and was never set on any thread, this flag is set to false,"] # [doc = " and OUTPUT_CAPTURE can be safely ignored on all threads, saving some time"] # [doc = " and memory registering an unused thread local."] # [doc = ""] # [doc = " Note about memory ordering: This contains information about whether a"] # [doc = " thread local variable might be in use. Although this is a global flag, the"] # [doc = " memory ordering between threads does not matter: we only want this flag to"] # [doc = " have a consistent order between set_output_capture and print_to *within"] # [doc = " the same thread*. Within the same thread, things always have a perfectly"] # [doc = " consistent order. So Ordering::Relaxed is fine."] static OUTPUT_CAPTURE_USED : Atomic < bool > = AtomicBool :: new (false) ;
};
}
