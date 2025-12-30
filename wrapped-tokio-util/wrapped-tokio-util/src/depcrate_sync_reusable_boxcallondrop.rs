// Generated macro for CallOnDrop (struct)
macro_rules! Depcrate_sync_reusable_boxCallOnDrop {
() => {
// Module: crate::sync::reusable_box
// Provides: {"CallOnDrop"}
// Dependencies: {}
struct CallOnDrop < O , F : FnOnce () -> O > { f : ManuallyDrop < F > , }
};
}
