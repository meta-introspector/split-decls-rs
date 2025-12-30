// Generated macro for ChannelInner (struct)
macro_rules! Depcrate_channelChannelInner {
() => {
// Module: crate::channel
// Provides: {"ChannelInner"}
// Dependencies: {}
struct ChannelInner { unsafe_raw : * mut raw :: LIBSSH2_CHANNEL , sess : Arc < Mutex < SessionInner > > , read_limit : Mutex < Option < u64 > > , }
};
}
