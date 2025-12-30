// Generated macro for impl_64 (impl)
macro_rules! Depcrate_commonimpl_64 {
() => {
// Module: crate::common
// Provides: {"impl_64"}
// Dependencies: {}
impl TlsState { # [inline] pub (crate) fn shutdown_read (& mut self) { match * self { Self :: WriteShutdown | Self :: FullyShutdown => * self = Self :: FullyShutdown , _ => * self = Self :: ReadShutdown , } } # [inline] pub (crate) fn shutdown_write (& mut self) { match * self { Self :: ReadShutdown | Self :: FullyShutdown => * self = Self :: FullyShutdown , _ => * self = Self :: WriteShutdown , } } # [inline] pub (crate) fn writeable (& self) -> bool { ! matches ! (* self , Self :: WriteShutdown | Self :: FullyShutdown) } # [inline] pub (crate) fn readable (& self) -> bool { ! matches ! (* self , Self :: ReadShutdown | Self :: FullyShutdown) } # [inline] # [cfg (feature = "early-data")] pub (crate) fn is_early_data (& self) -> bool { matches ! (self , Self :: EarlyData (..)) } # [inline] # [cfg (not (feature = "early-data"))] pub (crate) const fn is_early_data (& self) -> bool { false } }
};
}
