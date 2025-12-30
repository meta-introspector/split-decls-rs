// Generated macro for impl_1690 (impl)
macro_rules! Depcrate_streamimpl_1690 {
() => {
// Module: crate::stream
// Provides: {"impl_1690"}
// Dependencies: {}
impl < 'a , C , T , S > BufRead for Stream < 'a , C , T > where C : 'a + DerefMut + Deref < Target = ConnectionCommon < S > > , T : 'a + Read + Write , S : 'a + SideData , { fn fill_buf (& mut self) -> Result < & [u8] > { Stream { conn : self . conn , sock : self . sock , } . fill_buf () } fn consume (& mut self , amt : usize) { self . conn . reader () . consume (amt) } }
};
}
