// Generated macro for cfg_atomic_waker_impl (macro)
macro_rules! Depcrate_macros_cfgcfg_atomic_waker_impl {
() => {
// Module: crate::macros::cfg
// Provides: {"cfg_atomic_waker_impl"}
// Dependencies: {}
# [doc = " Enables internal `AtomicWaker` impl."] macro_rules ! cfg_atomic_waker_impl { ($ ($ item : item) *) => { $ (# [cfg (any (feature = "net" , feature = "process" , feature = "rt" , feature = "signal" , feature = "time" ,))] # [cfg (not (loom))] $ item) * } }
};
}
