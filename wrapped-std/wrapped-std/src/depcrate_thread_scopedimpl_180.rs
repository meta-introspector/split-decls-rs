// Generated macro for impl_180 (impl)
macro_rules! Depcrate_thread_scopedimpl_180 {
() => {
// Module: crate::thread::scoped
// Provides: {"impl_180"}
// Dependencies: {}
impl < 'scope , 'env > Scope < 'scope , 'env > { # [doc = " Spawns a new thread within a scope, returning a [`ScopedJoinHandle`] for it."] # [doc = ""] # [doc = " Unlike non-scoped threads, threads spawned with this function may"] # [doc = " borrow non-`'static` data from the outside the scope. See [`scope`] for"] # [doc = " details."] # [doc = ""] # [doc = " The join handle provides a [`join`] method that can be used to join the spawned"] # [doc = " thread. If the spawned thread panics, [`join`] will return an [`Err`] containing"] # [doc = " the panic payload."] # [doc = ""] # [doc = " If the join handle is dropped, the spawned thread will be implicitly joined at the"] # [doc = " end of the scope. In that case, if the spawned thread panics, [`scope`] will"] # [doc = " panic after all threads are joined."] # [doc = ""] # [doc = " This call will create a thread using default parameters of [`Builder`]."] # [doc = " If you want to specify the stack size or the name of the thread, use"] # [doc = " [`Builder::spawn_scoped`] instead."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the OS fails to create a thread; use [`Builder::spawn_scoped`]"] # [doc = " to recover from such errors."] # [doc = ""] # [doc = " [`join`]: ScopedJoinHandle::join"] # [stable (feature = "scoped_threads" , since = "1.63.0")] pub fn spawn < F , T > (& 'scope self , f : F) -> ScopedJoinHandle < 'scope , T > where F : FnOnce () -> T + Send + 'scope , T : Send + 'scope , { Builder :: new () . spawn_scoped (self , f) . expect ("failed to spawn thread") } }
};
}
