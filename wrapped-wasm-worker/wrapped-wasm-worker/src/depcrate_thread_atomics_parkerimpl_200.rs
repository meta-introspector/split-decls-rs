// Generated macro for impl_200 (impl)
macro_rules! Depcrate_thread_atomics_parkerimpl_200 {
() => {
// Module: crate::thread::atomics::parker
// Provides: {"impl_200"}
// Dependencies: {}
impl Parker { pub fn new (id : ThreadId) -> Self { Self { id , state : AtomicU32 :: new (EMPTY) , } } pub fn park (self : Pin < & Self >) { assert_eq ! (self . id , super :: current_id () , "called `park()` not from its corresponding thread") ; if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } loop { futex_wait (& self . state , PARKED , None) ; if self . state . compare_exchange (NOTIFIED , EMPTY , Acquire , Acquire) . is_ok () { return ; } else { } } } pub fn park_timeout (self : Pin < & Self > , timeout : Duration) { assert_eq ! (self . id , super :: current_id () , "called `park_timeout()` not from its corresponding thread") ; if self . state . fetch_sub (1 , Acquire) == NOTIFIED { return ; } futex_wait (& self . state , PARKED , Some (timeout)) ; if self . state . swap (EMPTY , Acquire) == NOTIFIED { } else { } } # [inline] pub fn unpark (self : Pin < & Self >) { if self . state . swap (NOTIFIED , Release) == PARKED { futex_wake (& self . state) ; } } }
};
}
