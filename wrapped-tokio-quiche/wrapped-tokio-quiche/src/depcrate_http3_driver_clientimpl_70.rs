// Generated macro for impl_70 (impl)
macro_rules! Depcrate_http3_driver_clientimpl_70 {
() => {
// Module: crate::http3::driver::client
// Provides: {"impl_70"}
// Dependencies: {}
impl ClientH3Controller { # [doc = " Creates a [`NewClientRequest`] sender for the paired [ClientH3Driver]."] pub fn request_sender (& self) -> ClientRequestSender { RequestSender { sender : self . cmd_sender . clone () , _r : Default :: default () , } } }
};
}
