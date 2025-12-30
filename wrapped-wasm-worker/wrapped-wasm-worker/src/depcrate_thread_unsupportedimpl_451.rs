// Generated macro for impl_451 (impl)
macro_rules! Depcrate_thread_unsupportedimpl_451 {
() => {
// Module: crate::thread::unsupported
// Provides: {"impl_451"}
// Dependencies: {}
impl < T > JoinHandle < T > { # [doc = " Implementation of [`std::thread::JoinHandle::is_finished()`]."] # [allow (clippy :: unused_self)] pub (super) fn is_finished (& self) -> bool { unreachable ! ("found instanced `JoinHandle` without threading support") } # [doc = " Implementation of [`std::thread::JoinHandle::join()`]."] # [allow (clippy :: unused_self)] pub (super) fn join (self) -> thread :: Result < T > { unreachable ! ("found instanced `JoinHandle` without threading support") } # [doc = " Implementation of [`std::thread::JoinHandle::thread()`]."] # [allow (clippy :: unused_self)] pub (super) fn thread (& self) -> & super :: Thread { unreachable ! ("found instanced `JoinHandle` without threading support") } # [doc = " Implementation for"] # [doc = " [`JoinHandleFuture::poll()`](crate::web::JoinHandleFuture)."] # [allow (clippy :: unused_self)] pub (super) fn poll (& self , _ : & mut Context < '_ >) -> Poll < thread :: Result < T > > { unreachable ! ("found instanced `JoinHandle` without threading support") } }
};
}
