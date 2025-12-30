// Generated macro for impl_60 (impl)
macro_rules! Depcrate_ansiimpl_60 {
() => {
// Module: crate::ansi
// Provides: {"impl_60"}
// Dependencies: {}
# [cfg (feature = "std")] impl Timeout for StdSyncHandler { # [inline] fn set_timeout (& mut self , duration : Duration) { self . timeout = Some (Instant :: now () + duration) ; } # [inline] fn clear_timeout (& mut self) { self . timeout = None ; } # [inline] fn pending_timeout (& self) -> bool { self . timeout . is_some () } }
};
}
