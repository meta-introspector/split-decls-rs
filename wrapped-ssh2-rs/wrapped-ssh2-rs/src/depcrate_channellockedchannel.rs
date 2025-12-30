// Generated macro for LockedChannel (struct)
macro_rules! Depcrate_channelLockedChannel {
() => {
// Module: crate::channel
// Provides: {"LockedChannel"}
// Dependencies: {}
struct LockedChannel < 'a > { raw : * mut raw :: LIBSSH2_CHANNEL , sess : MutexGuard < 'a , SessionInner > , }
};
}
