// Generated macro for impl_52 (impl)
macro_rules! Depcrate_channelimpl_52 {
() => {
// Module: crate::channel
// Provides: {"impl_52"}
// Dependencies: {}
impl Channel { pub (crate) fn from_raw_opt (raw : * mut raw :: LIBSSH2_CHANNEL , err : Option < Error > , sess : & Arc < Mutex < SessionInner > > ,) -> Result < Self , Error > { if raw . is_null () { Err (err . unwrap_or_else (Error :: unknown)) } else { Ok (Self { channel_inner : Arc :: new (ChannelInner { unsafe_raw : raw , sess : Arc :: clone (sess) , read_limit : Mutex :: new (None) , }) , }) } } fn lock (& self) -> LockedChannel { let sess = self . channel_inner . sess . lock () ; LockedChannel { sess , raw : self . channel_inner . unsafe_raw , } } }
};
}
