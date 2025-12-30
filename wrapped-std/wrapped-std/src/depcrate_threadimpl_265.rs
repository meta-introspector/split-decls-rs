// Generated macro for impl_265 (impl)
macro_rules! Depcrate_threadimpl_265 {
() => {
// Module: crate::thread
// Provides: {"impl_265"}
// Dependencies: {}
impl < 'scope , T > Drop for Packet < 'scope , T > { fn drop (& mut self) { let unhandled_panic = matches ! (self . result . get_mut () , Some (Err (_))) ; if let Err (_) = panic :: catch_unwind (panic :: AssertUnwindSafe (| | { * self . result . get_mut () = None ; })) { rtabort ! ("thread result panicked on drop") ; } if let Some (scope) = & self . scope { scope . decrement_num_running_threads (unhandled_panic) ; } } }
};
}
