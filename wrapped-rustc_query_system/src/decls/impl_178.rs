macro_rules! deps {
    () => {
        QueryJobId!();
        CycleError!();
        QueryWaiter!();
        QueryLatch!();
        QueryContext!();
        QueryLatchInfo!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < I > QueryLatch < I > { fn new () -> Self { QueryLatch { info : Arc :: new (Mutex :: new (QueryLatchInfo { complete : false , waiters : Vec :: new () })) , } } # [doc = " Awaits for the query job to complete."] pub (super) fn wait_on (& self , qcx : impl QueryContext , query : Option < QueryJobId > , span : Span ,) -> Result < () , CycleError < I > > { let waiter = Arc :: new (QueryWaiter { query , span , cycle : Mutex :: new (None) , condvar : Condvar :: new () }) ; self . wait_on_inner (qcx , & waiter) ; let mut cycle = waiter . cycle . lock () ; match cycle . take () { None => Ok (()) , Some (cycle) => Err (cycle) , } } # [doc = " Awaits the caller on this latch by blocking the current thread."] fn wait_on_inner (& self , qcx : impl QueryContext , waiter : & Arc < QueryWaiter < I > >) { let mut info = self . info . lock () ; if ! info . complete { info . waiters . push (Arc :: clone (waiter)) ; rustc_thread_pool :: mark_blocked () ; let proxy = qcx . jobserver_proxy () ; proxy . release_thread () ; waiter . condvar . wait (& mut info) ; drop (info) ; proxy . acquire_thread () ; } } # [doc = " Sets the latch and resumes all waiters on it"] fn set (& self) { let mut info = self . info . lock () ; debug_assert ! (! info . complete) ; info . complete = true ; let registry = rustc_thread_pool :: Registry :: current () ; for waiter in info . waiters . drain (..) { rustc_thread_pool :: mark_unblocked (& registry) ; waiter . condvar . notify_one () ; } } # [doc = " Removes a single waiter from the list of waiters."] # [doc = " This is used to break query cycles."] fn extract_waiter (& self , waiter : usize) -> Arc < QueryWaiter < I > > { let mut info = self . info . lock () ; debug_assert ! (! info . complete) ; info . waiters . remove (waiter) } }
    };
}

impl_178!();