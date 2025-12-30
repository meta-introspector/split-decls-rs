// Generated macro for Guard (struct)
macro_rules! Depcrate_guardGuard {
() => {
// Module: crate::guard
// Provides: {"Guard"}
// Dependencies: {}
# [doc = " [`Guard`] allows the user to read [`AtomicShared`](super::AtomicShared) and keeps the"] # [doc = " underlying instance pinned to the thread."] # [doc = ""] # [doc = " [`Guard`] internally prevents the global epoch value from passing through the value"] # [doc = " announced by the current thread, thus keeping reachable instances in the thread from being"] # [doc = " garbage collected."] # [derive (Debug)] pub struct Guard { collector_ptr : NonNull < Collector > , }
};
}
