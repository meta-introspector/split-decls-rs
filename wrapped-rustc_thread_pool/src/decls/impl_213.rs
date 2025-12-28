macro_rules! deps {
    () => {
        Counters!();
        JobsEventCounter!();
    };
}

macro_rules! impl_213 {
    () => {
        deps!();
        impl Counters { # [inline] fn new (word : usize) -> Counters { Counters { word } } # [inline] fn increment_jobs_counter (self) -> Counters { Counters { word : self . word . wrapping_add (ONE_JEC) } } # [inline] pub (super) fn jobs_counter (self) -> JobsEventCounter { JobsEventCounter (select_jec (self . word)) } # [doc = " The number of threads that are not actively"] # [doc = " executing work. They may be idle, sleepy, or asleep."] # [inline] pub (super) fn inactive_threads (self) -> usize { select_thread (self . word , INACTIVE_SHIFT) } # [inline] pub (super) fn awake_but_idle_threads (self) -> usize { debug_assert ! (self . sleeping_threads () <= self . inactive_threads () , "sleeping threads: {} > raw idle threads {}" , self . sleeping_threads () , self . inactive_threads ()) ; self . inactive_threads () - self . sleeping_threads () } # [inline] pub (super) fn sleeping_threads (self) -> usize { select_thread (self . word , SLEEPING_SHIFT) } }
    };
}

impl_213!()