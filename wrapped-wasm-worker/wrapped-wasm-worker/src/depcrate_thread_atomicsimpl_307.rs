// Generated macro for impl_307 (impl)
macro_rules! Depcrate_thread_atomicsimpl_307 {
() => {
// Module: crate::thread::atomics
// Provides: {"impl_307"}
// Dependencies: {}
impl < T > JoinHandle < T > { # [doc = " Implementation of [`std::thread::JoinHandle::is_finished()`]."] pub (super) fn is_finished (& self) -> bool { self . receiver . as_ref () . map_or (true , Receiver :: is_ready) } # [doc = " Implementation of [`std::thread::JoinHandle::join()`]."] # [allow (clippy :: unnecessary_wraps)] pub (super) fn join (self) -> thread :: Result < T > { assert_ne ! (self . thread () . id () , super :: current () . id () , "called `JoinHandle::join()` on the thread to join") ; Ok (self . receiver . expect ("`JoinHandle::join()` called after `JoinHandleFuture` polled to completion") . receive () . expect ("thread terminated without returning")) } # [doc = " Implementation of [`std::thread::JoinHandle::thread()`]."] # [allow (clippy :: missing_const_for_fn)] pub (super) fn thread (& self) -> & Thread { & self . thread } # [doc = " Implementation for"] # [doc = " [`JoinHandleFuture::poll()`](crate::web::JoinHandleFuture)."] pub (super) fn poll (& mut self , cx : & mut Context < '_ >) -> Poll < thread :: Result < T > > { assert_ne ! (self . thread () . id () , super :: current () . id () , "called `JoinHandle::join()` on the thread to join") ; let mut receiver = self . receiver . take () . expect ("`JoinHandleFuture` polled or created after completion") ; match Pin :: new (& mut receiver) . poll (cx) { Poll :: Ready (Some (value)) => Poll :: Ready (Ok (value)) , Poll :: Pending => { self . receiver = Some (receiver) ; Poll :: Pending } Poll :: Ready (None) => unreachable ! ("thread terminated without returning") , } } }
};
}
