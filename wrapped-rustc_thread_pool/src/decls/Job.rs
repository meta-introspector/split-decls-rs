macro_rules! Job {
    () => {
        # [doc = " A `Job` is used to advertise work for other threads that they may"] # [doc = " want to steal. In accordance with time honored tradition, jobs are"] # [doc = " arranged in a deque, so that thieves can take from the top of the"] # [doc = " deque while the main worker manages the bottom of the deque. This"] # [doc = " deque is managed by the `thread_pool` module."] pub (super) trait Job { # [doc = " Unsafe: this may be called from a different thread than the one"] # [doc = " which scheduled the job, so the implementer must ensure the"] # [doc = " appropriate traits are met, whether `Send`, `Sync`, or both."] unsafe fn execute (this : * const ()) ; }
    };
}

Job!()