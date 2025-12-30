// Generated macro for impl_731 (impl)
macro_rules! Depcrate_conn_unbufferedimpl_731 {
() => {
// Module: crate::conn::unbuffered
// Provides: {"impl_731"}
// Dependencies: {}
impl < Side : SideData > TransmitTlsData < '_ , Side > { # [doc = " Signals that the previously encoded TLS data has been transmitted"] pub fn done (self) { self . conn . wants_write = false ; } # [doc = " Returns an adapter that allows encrypting application data"] # [doc = ""] # [doc = " If allowed at this stage of the handshake process"] pub fn may_encrypt_app_data (& mut self) -> Option < WriteTraffic < '_ , Side > > { if self . conn . core . common_state . may_send_application_data { Some (WriteTraffic { conn : self . conn }) } else { None } } }
};
}
