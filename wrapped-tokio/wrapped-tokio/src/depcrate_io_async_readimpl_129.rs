// Generated macro for impl_129 (impl)
macro_rules! Depcrate_io_async_readimpl_129 {
() => {
// Module: crate::io::async_read
// Provides: {"impl_129"}
// Dependencies: {}
impl < T : AsRef < [u8] > + Unpin > AsyncRead for io :: Cursor < T > { fn poll_read (mut self : Pin < & mut Self > , _cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let pos = self . position () ; let slice : & [u8] = (* self) . get_ref () . as_ref () ; if pos > slice . len () as u64 { return Poll :: Ready (Ok (())) ; } let start = pos as usize ; let amt = std :: cmp :: min (slice . len () - start , buf . remaining ()) ; let end = start + amt ; buf . put_slice (& slice [start .. end]) ; self . set_position (end as u64) ; Poll :: Ready (Ok (())) } }
};
}
