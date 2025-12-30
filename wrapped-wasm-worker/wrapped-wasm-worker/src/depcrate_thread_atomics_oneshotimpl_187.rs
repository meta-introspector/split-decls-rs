// Generated macro for impl_187 (impl)
macro_rules! Depcrate_thread_atomics_oneshotimpl_187 {
() => {
// Module: crate::thread::atomics::oneshot
// Provides: {"impl_187"}
// Dependencies: {}
impl < T > Future for Receiver < T > { type Output = Option < T > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let Some (state) = self . 0 . take () else { panic ! ("polled after completion") } ; let mut value = match state . value . try_lock () { Ok (mut value) => value . take () , Err (TryLockError :: Poisoned (error)) => error . into_inner () . take () , Err (TryLockError :: WouldBlock) => None , } ; if value . is_none () { state . waker . register (cx . waker ()) ; value = match state . value . try_lock () { Ok (mut value) => value . take () , Err (TryLockError :: Poisoned (error)) => error . into_inner () . take () , Err (TryLockError :: WouldBlock) => None , } ; } if let Some (state) = value { match state { State :: Result (value) => Poll :: Ready (Some (value)) , State :: Dropped => Poll :: Ready (None) , State :: Waiting => unreachable ! ("wrong state returns by `State::take()`") , State :: Taken => unreachable ! ("falsely inserted wrong state") , } } else { self . 0 = Some (state) ; Poll :: Pending } } }
};
}
