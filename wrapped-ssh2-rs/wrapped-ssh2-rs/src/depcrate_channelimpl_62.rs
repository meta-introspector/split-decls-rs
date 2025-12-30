// Generated macro for impl_62 (impl)
macro_rules! Depcrate_channelimpl_62 {
() => {
// Module: crate::channel
// Provides: {"impl_62"}
// Dependencies: {}
impl Drop for ChannelInner { fn drop (& mut self) { unsafe { let _ = raw :: libssh2_channel_free (self . unsafe_raw) ; } } }
};
}
