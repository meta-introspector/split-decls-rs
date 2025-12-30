// Generated macro for ArcJob (struct)
macro_rules! Depcrate_jobArcJob {
() => {
// Module: crate::job
// Provides: {"ArcJob"}
// Dependencies: {}
# [doc = " Represents a job stored in an `Arc` -- like `HeapJob`, but may"] # [doc = " be turned into multiple `JobRef`s and called multiple times."] pub (super) struct ArcJob < BODY > where BODY : Fn (JobRefId) + Send + Sync , { job : BODY , }
};
}
