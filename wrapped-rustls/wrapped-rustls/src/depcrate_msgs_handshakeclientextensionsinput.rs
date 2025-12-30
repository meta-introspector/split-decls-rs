// Generated macro for ClientExtensionsInput (struct)
macro_rules! Depcrate_msgs_handshakeClientExtensionsInput {
() => {
// Module: crate::msgs::handshake
// Provides: {"ClientExtensionsInput"}
// Dependencies: {}
# [doc = " A precursor to `ClientExtensions`, allowing customisation."] # [doc = ""] # [doc = " This is smaller than `ClientExtensions`, as it only contains the extensions"] # [doc = " we need to vary between different protocols (eg, TCP-TLS versus QUIC)."] # [derive (Clone , Default)] pub (crate) struct ClientExtensionsInput < 'a > { # [doc = " QUIC transport parameters"] pub (crate) transport_parameters : Option < TransportParameters < 'a > > , # [doc = " ALPN protocols"] pub (crate) protocols : Option < Vec < ProtocolName > > , }
};
}
