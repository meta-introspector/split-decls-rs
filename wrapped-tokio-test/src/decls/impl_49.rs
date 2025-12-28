macro_rules! deps {
    () => {
        ThreadWaker!();
        MockTask!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        impl MockTask { # [doc = " Creates new mock task"] fn new () -> Self { MockTask { waker : Arc :: new (ThreadWaker :: new ()) , } } # [doc = " Runs a closure from the context of the task."] # [doc = ""] # [doc = " Any wake notifications resulting from the execution of the closure are"] # [doc = " tracked."] fn enter < F , R > (& mut self , f : F) -> R where F : FnOnce (& mut Context < '_ >) -> R , { self . waker . clear () ; let waker = self . waker () ; let mut cx = Context :: from_waker (& waker) ; f (& mut cx) } # [doc = " Returns `true` if the inner future has received a wake notification"] # [doc = " since the last call to `enter`."] fn is_woken (& self) -> bool { self . waker . is_woken () } # [doc = " Returns the number of references to the task waker"] # [doc = ""] # [doc = " The task itself holds a reference. The return value will never be zero."] fn waker_ref_count (& self) -> usize { Arc :: strong_count (& self . waker) } fn waker (& self) -> Waker { unsafe { let raw = to_raw (self . waker . clone ()) ; Waker :: from_raw (raw) } } }
    };
}

impl_49!()