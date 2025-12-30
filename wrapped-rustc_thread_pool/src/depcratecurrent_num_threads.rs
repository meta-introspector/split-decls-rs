// Generated macro for current_num_threads (function)
macro_rules! Depcratecurrent_num_threads {
() => {
// Module: crate
// Provides: {"current_num_threads"}
// Dependencies: {}
# [doc = " Returns the number of threads in the current registry. If this"] # [doc = " code is executing within a Rayon thread-pool, then this will be"] # [doc = " the number of threads for the thread-pool of the current"] # [doc = " thread. Otherwise, it will be the number of threads for the global"] # [doc = " thread-pool."] # [doc = ""] # [doc = " This can be useful when trying to judge how many times to split"] # [doc = " parallel work (the parallel iterator traits use this value"] # [doc = " internally for this purpose)."] # [doc = ""] # [doc = " # Future compatibility note"] # [doc = ""] # [doc = " Note that unless this thread-pool was created with a"] # [doc = " builder that specifies the number of threads, then this"] # [doc = " number may vary over time in future versions (see [the"] # [doc = " `num_threads()` method for details][snt])."] # [doc = ""] # [doc = " [snt]: struct.ThreadPoolBuilder.html#method.num_threads"] pub fn current_num_threads () -> usize { crate :: registry :: Registry :: current_num_threads () }
};
}
