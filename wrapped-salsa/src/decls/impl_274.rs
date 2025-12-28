macro_rules! deps {
    () => {
        Zalsa!();
        Running!();
        EventKind!();
        Cancelled!();
        BlockedOnInner!();
        WaitResult!();
        Event!();
    };
}

macro_rules! impl_274 {
    () => {
        deps!();
        impl Running < '_ > { # [doc = " Blocks on the other thread to complete the computation."] pub (crate) fn block_on (self , zalsa : & Zalsa) { let BlockedOnInner { dg , query_mutex_guard , database_key , other_id , thread_id , } = * self . 0 ; zalsa . event (& | | { Event :: new (EventKind :: WillBlockOn { other_thread_id : other_id , database_key , }) }) ; crate :: tracing :: info ! ("block_on: thread {thread_id:?} is blocking on {database_key:?} in thread {other_id:?}" ,) ; let result = DependencyGraph :: block_on (dg , thread_id , database_key , other_id , query_mutex_guard) ; match result { WaitResult :: Panicked => { Cancelled :: PropagatedPanic . throw () } WaitResult :: Completed => { } } } }
    };
}

impl_274!();