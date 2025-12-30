// Generated macro for impl_67 (impl)
macro_rules! Depcrate_commonimpl_67 {
() => {
// Module: crate::common
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'a , IO : AsyncRead + AsyncWrite + Unpin , C , SD > AsyncRead for Stream < 'a , IO , C > where C : DerefMut + Deref < Target = ConnectionCommon < SD > > , SD : SideData + 'a , { fn poll_read (mut self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : & mut ReadBuf < '_ > ,) -> Poll < io :: Result < () > > { let data = ready ! (self . as_mut () . poll_fill_buf (cx)) ? ; let amount = buf . remaining () . min (data . len ()) ; buf . put_slice (& data [.. amount]) ; self . session . reader () . consume (amount) ; Poll :: Ready (Ok (())) } }
};
}
