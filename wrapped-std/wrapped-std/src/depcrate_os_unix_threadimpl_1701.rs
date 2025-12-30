// Generated macro for impl_1701 (impl)
macro_rules! Depcrate_os_unix_threadimpl_1701 {
() => {
// Module: crate::os::unix::thread
// Provides: {"impl_1701"}
// Dependencies: {}
# [stable (feature = "thread_extensions" , since = "1.9.0")] impl < T > JoinHandleExt for JoinHandle < T > { fn as_pthread_t (& self) -> RawPthread { self . as_inner () . id () as RawPthread } fn into_pthread_t (self) -> RawPthread { self . into_inner () . into_id () as RawPthread } }
};
}
