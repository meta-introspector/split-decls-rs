use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Used to create a new [`ThreadPool`] or to configure the global rayon thread pool.
/// ## Creating a ThreadPool
/// The following creates a thread pool with 22 threads.
///
/// ```rust
/// # use rustc_thread_pool as rayon;
/// let pool = rayon::ThreadPoolBuilder::new().num_threads(22).build().unwrap();
/// ```
///
/// To instead configure the global thread pool, use [`build_global()`]:
///
/// ```rust
/// # use rustc_thread_pool as rayon;
/// rayon::ThreadPoolBuilder::new().num_threads(22).build_global().unwrap();
/// ```
///
/// [`ThreadPool`]: struct.ThreadPool.html
/// [`build_global()`]: struct.ThreadPoolBuilder.html#method.build_global
pub struct ThreadPoolBuilder<S = DefaultSpawn> {
    /// The number of threads in the rayon thread pool.
    /// If zero will use the RAYON_NUM_THREADS environment variable.
    /// If RAYON_NUM_THREADS is invalid or zero will use the default.
    num_threads: usize,
    /// Custom closure, if any, to handle a panic that we cannot propagate
    /// anywhere else.
    panic_handler: Option<Box<PanicHandler>>,
    /// Closure to compute the name of a thread.
    get_thread_name: Option<Box<dyn FnMut(usize) -> String>>,
    /// The stack size for the created worker threads
    stack_size: Option<usize>,
    /// Closure invoked on deadlock.
    deadlock_handler: Option<Box<DeadlockHandler>>,
    /// Closure invoked on worker thread start.
    start_handler: Option<Box<StartHandler>>,
    /// Closure invoked on worker thread exit.
    exit_handler: Option<Box<ExitHandler>>,
    /// Closure invoked to spawn threads.
    spawn_handler: S,
    /// Closure invoked when starting computations in a thread.
    acquire_thread_handler: Option<Box<AcquireThreadHandler>>,
    /// Closure invoked when blocking in a thread.
    release_thread_handler: Option<Box<ReleaseThreadHandler>>,
    /// If false, worker threads will execute spawned jobs in a
    /// "depth-first" fashion. If true, they will do a "breadth-first"
    /// fashion. Depth-first is the default.
    breadth_first: bool,
}
