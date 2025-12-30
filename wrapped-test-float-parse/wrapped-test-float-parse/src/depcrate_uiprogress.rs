// Generated macro for Progress (struct)
macro_rules! Depcrate_uiProgress {
() => {
// Module: crate::ui
// Provides: {"Progress"}
// Dependencies: {}
# [doc = " Thin abstraction over our usage of a `ProgressBar`."] # [derive (Debug)] pub struct Progress { pb : ProgressBar , make_final_style : NoDebug < Box < dyn Fn (& 'static str) -> ProgressStyle + Sync > > , }
};
}
