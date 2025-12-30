// Generated macro for max_num_threads (function)
macro_rules! Depcratemax_num_threads {
() => {
// Module: crate
// Provides: {"max_num_threads"}
// Dependencies: {}
# [doc = " Returns the maximum number of threads that Rayon supports in a single thread-pool."] # [doc = ""] # [doc = " If a higher thread count is requested by calling `ThreadPoolBuilder::num_threads` or by setting"] # [doc = " the `RAYON_NUM_THREADS` environment variable, then it will be reduced to this maximum."] # [doc = ""] # [doc = " The value may vary between different targets, and is subject to change in new Rayon versions."] pub fn max_num_threads () -> usize { crate :: sleep :: THREADS_MAX }
};
}
