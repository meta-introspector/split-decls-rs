// Generated macro for impl_478 (impl)
macro_rules! Depcrate_thread_yield_nowimpl_478 {
() => {
// Module: crate::thread::yield_now
// Provides: {"impl_478"}
// Dependencies: {}
impl Future for YieldNowFuture { type Output = () ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match self . 0 . as_mut () . expect ("`YieldNowFuture` polled after completion") { State :: Scheduler { future , .. } => { ready ! (Pin :: new (future) . poll (cx)) . expect ("unexpected failure in empty `Promise`") ; } State :: Idle { waker , .. } | State :: Channel { waker , .. } => { ready ! (waker . borrow_mut () . poll (cx)) ; } State :: None => () , } self . 0 . take () . expect ("found empty `State`") ; Poll :: Ready (()) } }
};
}
