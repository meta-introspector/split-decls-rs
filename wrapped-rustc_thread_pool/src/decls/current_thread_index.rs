macro_rules! deps {
    () => {
        WorkerThread!();
    };
}

macro_rules! current_thread_index {
    () => {
        deps!();
        # [doc = " If called from a Rayon worker thread, returns the index of that"] # [doc = " thread within its current pool; if not called from a Rayon thread,"] # [doc = " returns `None`."] # [doc = ""] # [doc = " The index for a given thread will not change over the thread's"] # [doc = " lifetime. However, multiple threads may share the same index if"] # [doc = " they are in distinct thread-pools."] # [doc = ""] # [doc = " See also: [the `ThreadPool::current_thread_index()` method]."] # [doc = ""] # [doc = " [m]: struct.ThreadPool.html#method.current_thread_index"] # [doc = ""] # [doc = " # Future compatibility note"] # [doc = ""] # [doc = " Currently, every thread-pool (including the global"] # [doc = " thread-pool) has a fixed number of threads, but this may"] # [doc = " change in future Rayon versions (see [the `num_threads()` method"] # [doc = " for details][snt]). In that case, the index for a"] # [doc = " thread would not change during its lifetime, but thread"] # [doc = " indices may wind up being reused if threads are terminated and"] # [doc = " restarted."] # [doc = ""] # [doc = " [snt]: struct.ThreadPoolBuilder.html#method.num_threads"] # [inline] pub fn current_thread_index () -> Option < usize > { unsafe { let curr = WorkerThread :: current () . as_ref () ? ; Some (curr . index ()) } }
    };
}

current_thread_index!()