// Generated macro for LockedStream (struct)
macro_rules! Depcrate_channelLockedStream {
() => {
// Module: crate::channel
// Provides: {"LockedStream"}
// Dependencies: {}
struct LockedStream < 'a > { raw : * mut raw :: LIBSSH2_CHANNEL , sess : MutexGuard < 'a , SessionInner > , id : i32 , read_limit : MutexGuard < 'a , Option < u64 > > , }
};
}
