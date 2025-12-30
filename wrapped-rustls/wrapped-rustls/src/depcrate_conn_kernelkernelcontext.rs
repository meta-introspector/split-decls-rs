// Generated macro for KernelContext (struct)
macro_rules! Depcrate_conn_kernelKernelContext {
() => {
// Module: crate::conn::kernel
// Provides: {"KernelContext"}
// Dependencies: {}
pub (crate) struct KernelContext < 'a > { pub (crate) peer_identity : Option < & 'a Identity < 'static > > , pub (crate) protocol : Protocol , pub (crate) quic : & 'a Quic , }
};
}
