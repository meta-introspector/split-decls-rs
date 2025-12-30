// Generated macro for yield_local (function)
macro_rules! Depcrate_thread_poolyield_local {
() => {
// Module: crate::thread_pool
// Provides: {"yield_local"}
// Dependencies: {}
# [doc = " Cooperatively yields execution to local Rayon work."] # [doc = ""] # [doc = " If the current thread is part of a rayon thread pool, this looks for a"] # [doc = " single unit of pending work in this thread's queue, then executes it."] # [doc = " Completion of that work might include nested work or further work stealing."] # [doc = ""] # [doc = " This is similar to [`yield_now()`], but does not steal from other threads."] # [doc = ""] # [doc = " Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if"] # [doc = " nothing was available, or `None` if this thread is not part of any pool at all."] pub fn yield_local () -> Option < Yield > { unsafe { let thread = WorkerThread :: current () . as_ref () ? ; Some (thread . yield_local ()) } }
};
}
