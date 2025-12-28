macro_rules! deps {
    () => {
        CoreLatch!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl CoreLatch { # [inline] fn new () -> Self { Self { state : AtomicUsize :: new (0) } } # [doc = " Invoked by owning thread as it prepares to sleep. Returns true"] # [doc = " if the owning thread may proceed to fall asleep, false if the"] # [doc = " latch was set in the meantime."] # [inline] pub (super) fn get_sleepy (& self) -> bool { self . state . compare_exchange (UNSET , SLEEPY , Ordering :: SeqCst , Ordering :: Relaxed) . is_ok () } # [doc = " Invoked by owning thread as it falls asleep sleep. Returns"] # [doc = " true if the owning thread should block, or false if the latch"] # [doc = " was set in the meantime."] # [inline] pub (super) fn fall_asleep (& self) -> bool { self . state . compare_exchange (SLEEPY , SLEEPING , Ordering :: SeqCst , Ordering :: Relaxed) . is_ok () } # [doc = " Invoked by owning thread as it falls asleep sleep. Returns"] # [doc = " true if the owning thread should block, or false if the latch"] # [doc = " was set in the meantime."] # [inline] pub (super) fn wake_up (& self) { if ! self . probe () { let _ = self . state . compare_exchange (SLEEPING , UNSET , Ordering :: SeqCst , Ordering :: Relaxed) ; } } # [doc = " Set the latch. If this returns true, the owning thread was sleeping"] # [doc = " and must be awoken."] # [doc = ""] # [doc = " This is private because, typically, setting a latch involves"] # [doc = " doing some wakeups; those are encapsulated in the surrounding"] # [doc = " latch code."] # [inline] unsafe fn set (this : * const Self) -> bool { let old_state = unsafe { (* this) . state . swap (SET , Ordering :: AcqRel) } ; old_state == SLEEPING } # [doc = " Test if this latch has been set."] # [inline] pub (super) fn probe (& self) -> bool { self . state . load (Ordering :: Acquire) == SET } }
    };
}

impl_75!()