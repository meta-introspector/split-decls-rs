// Generated macro for impl_1694 (impl)
macro_rules! Depcrate_streamimpl_1694 {
() => {
// Module: crate::stream
// Provides: {"impl_1694"}
// Dependencies: {}
impl < 'a , C , T , S > StreamOwned < C , T > where C : DerefMut + Deref < Target = ConnectionCommon < S > > , T : Read + Write , S : SideData , { fn as_stream (& 'a mut self) -> Stream < 'a , C , T > { Stream { conn : & mut self . conn , sock : & mut self . sock , } } }
};
}
