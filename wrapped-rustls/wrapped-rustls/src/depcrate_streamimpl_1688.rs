// Generated macro for impl_1688 (impl)
macro_rules! Depcrate_streamimpl_1688 {
() => {
// Module: crate::stream
// Provides: {"impl_1688"}
// Dependencies: {}
impl < 'a , C , T , S > Stream < 'a , C , T > where C : 'a + DerefMut + Deref < Target = ConnectionCommon < S > > , T : 'a + Read + Write , S : SideData , { # [doc = " Make a new Stream using the Connection `conn` and socket-like object"] # [doc = " `sock`.  This does not fail and does no IO."] pub fn new (conn : & 'a mut C , sock : & 'a mut T) -> Self { Self { conn , sock } } # [doc = " If we're handshaking, complete all the IO for that."] # [doc = " If we have data to write, write it all."] fn complete_prior_io (& mut self) -> Result < () > { if self . conn . is_handshaking () { self . conn . complete_io (self . sock) ? ; } if self . conn . wants_write () { self . conn . complete_io (self . sock) ? ; } Ok (()) } fn prepare_read (& mut self) -> Result < () > { self . complete_prior_io () ? ; while self . conn . wants_read () { if self . conn . complete_io (self . sock) ? . 0 == 0 { break ; } } Ok (()) } fn fill_buf (mut self) -> Result < & 'a [u8] > where S : 'a , { self . prepare_read () ? ; self . conn . reader () . into_first_chunk () } }
};
}
