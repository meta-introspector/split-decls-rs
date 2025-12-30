// Generated macro for impl_68 (impl)
macro_rules! Depcrate_commonimpl_68 {
() => {
// Module: crate::common
// Provides: {"impl_68"}
// Dependencies: {}
impl < 'a , IO : AsyncRead + AsyncWrite + Unpin , C , SD > AsyncBufRead for Stream < 'a , IO , C > where C : DerefMut + Deref < Target = ConnectionCommon < SD > > , SD : SideData + 'a , { fn poll_fill_buf (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < io :: Result < & [u8] > > { let this = self . get_mut () ; Stream { io : this . io , session : this . session , .. * this } . poll_fill_buf (cx) } fn consume (mut self : Pin < & mut Self > , amt : usize) { self . session . reader () . consume (amt) ; } }
};
}
