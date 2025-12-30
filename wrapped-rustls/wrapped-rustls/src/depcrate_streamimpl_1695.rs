// Generated macro for impl_1695 (impl)
macro_rules! Depcrate_streamimpl_1695 {
() => {
// Module: crate::stream
// Provides: {"impl_1695"}
// Dependencies: {}
impl < C , T , S > Read for StreamOwned < C , T > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , T : Read + Write , S : SideData , { fn read (& mut self , buf : & mut [u8]) -> Result < usize > { self . as_stream () . read (buf) } }
};
}
