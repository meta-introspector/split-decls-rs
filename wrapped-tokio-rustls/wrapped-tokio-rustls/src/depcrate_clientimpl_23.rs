// Generated macro for impl_23 (impl)
macro_rules! Depcrate_clientimpl_23 {
() => {
// Module: crate::client
// Provides: {"impl_23"}
// Dependencies: {}
impl From < Arc < ClientConfig > > for TlsConnector { fn from (inner : Arc < ClientConfig >) -> Self { Self { inner , # [cfg (feature = "early-data")] early_data : false , } } }
};
}
