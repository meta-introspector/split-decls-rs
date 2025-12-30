// Generated macro for impl_1696 (impl)
macro_rules! Depcrate_streamimpl_1696 {
() => {
// Module: crate::stream
// Provides: {"impl_1696"}
// Dependencies: {}
impl < C , T , S > BufRead for StreamOwned < C , T > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , T : Read + Write , S : 'static + SideData , { fn fill_buf (& mut self) -> Result < & [u8] > { self . as_stream () . fill_buf () } fn consume (& mut self , amt : usize) { self . as_stream () . consume (amt) } }
};
}
