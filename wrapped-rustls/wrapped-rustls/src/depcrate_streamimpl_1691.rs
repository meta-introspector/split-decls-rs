// Generated macro for impl_1691 (impl)
macro_rules! Depcrate_streamimpl_1691 {
() => {
// Module: crate::stream
// Provides: {"impl_1691"}
// Dependencies: {}
impl < 'a , C , T , S > Write for Stream < 'a , C , T > where C : 'a + DerefMut + Deref < Target = ConnectionCommon < S > > , T : 'a + Read + Write , S : SideData , { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . complete_prior_io () ? ; let len = self . conn . writer () . write (buf) ? ; let _ = self . conn . complete_io (self . sock) ; Ok (len) } fn write_vectored (& mut self , bufs : & [IoSlice < '_ >]) -> Result < usize > { self . complete_prior_io () ? ; let len = self . conn . writer () . write_vectored (bufs) ? ; let _ = self . conn . complete_io (self . sock) ; Ok (len) } fn flush (& mut self) -> Result < () > { self . complete_prior_io () ? ; self . conn . writer () . flush () ? ; if self . conn . wants_write () { self . conn . complete_io (self . sock) ? ; } Ok (()) } }
};
}
