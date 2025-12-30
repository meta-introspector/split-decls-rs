// Generated macro for impl_55 (impl)
macro_rules! Depcrate_channelimpl_55 {
() => {
// Module: crate::channel
// Provides: {"impl_55"}
// Dependencies: {}
impl < 'a > LockedStream < 'a > { pub fn eof (& self) -> bool { * self . read_limit == Some (0) || unsafe { raw :: libssh2_channel_eof (self . raw) != 0 } } }
};
}
