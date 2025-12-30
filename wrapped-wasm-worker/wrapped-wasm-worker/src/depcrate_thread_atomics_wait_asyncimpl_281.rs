// Generated macro for impl_281 (impl)
macro_rules! Depcrate_thread_atomics_wait_asyncimpl_281 {
() => {
// Module: crate::thread::atomics::wait_async
// Provides: {"impl_281"}
// Dependencies: {}
impl Future for WaitAsync { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let state = self . 0 . as_mut () . expect ("`WaitAsync` polled after completion") ; match state { State :: Ready => { self . 0 . take () ; Poll :: Ready (()) } State :: WaitAsync (future) => { ready ! (Pin :: new (future) . poll (cx)) . expect ("`Promise` returned by `Atomics.waitAsync` should never throw") ; self . 0 . take () ; Poll :: Ready (()) } State :: Polyfill (shared) => { if shared . finished . get () { self . 0 . take () ; Poll :: Ready (()) } else { * shared . waker . borrow_mut () = Some (cx . waker () . clone ()) ; Poll :: Pending } } } } }
};
}
