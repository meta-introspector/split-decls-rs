// Generated macro for JoinHandleExt (trait)
macro_rules! Depcrate_os_unix_threadJoinHandleExt {
() => {
// Module: crate::os::unix::thread
// Provides: {"JoinHandleExt"}
// Dependencies: {}
# [doc = " Unix-specific extensions to [`JoinHandle`]."] # [stable (feature = "thread_extensions" , since = "1.9.0")] pub trait JoinHandleExt { # [doc = " Extracts the raw pthread_t without taking ownership"] # [stable (feature = "thread_extensions" , since = "1.9.0")] fn as_pthread_t (& self) -> RawPthread ; # [doc = " Consumes the thread, returning the raw pthread_t"] # [doc = ""] # [doc = " This function **transfers ownership** of the underlying pthread_t to"] # [doc = " the caller. Callers are then the unique owners of the pthread_t and"] # [doc = " must either detach or join the pthread_t once it's no longer needed."] # [stable (feature = "thread_extensions" , since = "1.9.0")] fn into_pthread_t (self) -> RawPthread ; }
};
}
