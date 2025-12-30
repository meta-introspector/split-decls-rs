// Generated macro for KernelConnection (struct)
macro_rules! Depcrate_conn_kernelKernelConnection {
() => {
// Module: crate::conn::kernel
// Provides: {"KernelConnection"}
// Dependencies: {}
# [doc = " A kernel connection."] # [doc = ""] # [doc = " This does not directly wrap a kernel connection, rather it gives you the"] # [doc = " minimal interfaces you need to implement a well-behaved TLS connection on"] # [doc = " top of kTLS."] # [doc = ""] # [doc = " See the [`crate::kernel`] module docs for more details."] pub struct KernelConnection < Side > { state : Box < dyn KernelState > , peer_identity : Option < Identity < 'static > > , quic : Quic , negotiated_version : ProtocolVersion , protocol : Protocol , suite : SupportedCipherSuite , _side : PhantomData < Side > , }
};
}
