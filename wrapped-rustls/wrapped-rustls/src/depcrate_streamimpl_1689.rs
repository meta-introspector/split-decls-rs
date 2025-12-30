// Generated macro for impl_1689 (impl)
macro_rules! Depcrate_streamimpl_1689 {
() => {
// Module: crate::stream
// Provides: {"impl_1689"}
// Dependencies: {}
impl < 'a , C , T , S > Read for Stream < 'a , C , T > where C : 'a + DerefMut + Deref < Target = ConnectionCommon < S > > , T : 'a + Read + Write , S : SideData , { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { self . prepare_read () ? ; self . conn . reader () . read (buf) } }
};
}
