macro_rules! THREADS_MAX {
    () => {
        # [doc = " Max value for the thread counters."] pub (crate) const THREADS_MAX : usize = (1 << THREADS_BITS) - 1 ;
    };
}

THREADS_MAX!();