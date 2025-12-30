// Generated macro for impl_63 (impl)
macro_rules! Depcrate_channelimpl_63 {
() => {
// Module: crate::channel
// Provides: {"impl_63"}
// Dependencies: {}
impl Stream { fn lock (& self) -> LockedStream { let sess = self . channel_inner . sess . lock () ; LockedStream { sess , raw : self . channel_inner . unsafe_raw , id : self . id , read_limit : self . channel_inner . read_limit . lock () , } } }
};
}
