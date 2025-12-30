// Generated macro for impl_385 (impl)
macro_rules! Depcrate_thread_scopeimpl_385 {
() => {
// Module: crate::thread::scope
// Provides: {"impl_385"}
// Dependencies: {}
# [pinned_drop] impl < F , T > PinnedDrop for ScopeFuture < '_ , '_ , F , T > { fn drop (self : Pin < & mut Self >) { let this = self . project () ; if let ScopeFutureReplace :: Task { scope , .. } | ScopeFutureReplace :: Wait { scope , .. } = this . 0 . project_replace (State :: None) { scope . this . finish () ; } } }
};
}
