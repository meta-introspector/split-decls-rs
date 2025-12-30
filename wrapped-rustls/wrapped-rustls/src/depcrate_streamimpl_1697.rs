// Generated macro for impl_1697 (impl)
macro_rules! Depcrate_streamimpl_1697 {
() => {
// Module: crate::stream
// Provides: {"impl_1697"}
// Dependencies: {}
impl < C , T , S > Write for StreamOwned < C , T > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , T : Read + Write , S : SideData , { fn write (& mut self , buf : & [u8]) -> Result < usize > { self . as_stream () . write (buf) } fn flush (& mut self) -> Result < () > { self . as_stream () . flush () } }
};
}
