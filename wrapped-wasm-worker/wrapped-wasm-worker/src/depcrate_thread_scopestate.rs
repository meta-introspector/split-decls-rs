// Generated macro for State (enum)
macro_rules! Depcrate_thread_scopeState {
() => {
// Module: crate::thread::scope
// Provides: {"State"}
// Dependencies: {}
# [doc = " State for [`ScopeFuture`]."] # [pin_project (project = ScopeFutureProj , project_replace = ScopeFutureReplace)] enum State < 'scope , 'env , F , T > { # [doc = " Executing the task given to [`scope_async()`]."] Task { # [doc = " [`Future`] given by the caller."] # [pin] task : F , # [doc = " Corresponding [`Scope`]."] scope : Pin < Box < Scope < 'scope , 'env > > > , } , # [doc = " Wait for all threads to finish."] Wait { # [doc = " Result of the [`Future`] given by the caller."] result : T , # [doc = " Corresponding [`Scope`]."] scope : Pin < Box < Scope < 'scope , 'env > > > , } , # [doc = " [`Future`] was polled to conclusion."] None , }
};
}
