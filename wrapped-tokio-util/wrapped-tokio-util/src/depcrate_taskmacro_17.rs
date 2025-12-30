// Generated macro for macro_17 (macro)
macro_rules! Depcrate_taskmacro_17 {
() => {
// Module: crate::task
// Provides: {"macro_17"}
// Dependencies: {}
cfg_rt ! { mod spawn_pinned ; pub use spawn_pinned :: LocalPoolHandle ; pub mod task_tracker ; # [doc (inline)] pub use task_tracker :: TaskTracker ; mod abort_on_drop ; pub use abort_on_drop :: AbortOnDropHandle ; mod join_queue ; pub use join_queue :: JoinQueue ; }
};
}
