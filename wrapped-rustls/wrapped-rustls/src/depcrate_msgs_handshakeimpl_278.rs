// Generated macro for impl_278 (impl)
macro_rules! Depcrate_msgs_handshakeimpl_278 {
() => {
// Module: crate::msgs::handshake
// Provides: {"impl_278"}
// Dependencies: {}
impl ClientExtensionsInput < '_ > { pub (crate) fn from_alpn (alpn_protocols : Vec < Vec < u8 > >) -> ClientExtensionsInput < 'static > { let protocols = match alpn_protocols . is_empty () { true => None , false => Some (alpn_protocols . into_iter () . map (ProtocolName :: from) . collect :: < Vec < _ > > () ,) , } ; ClientExtensionsInput { transport_parameters : None , protocols , } } pub (crate) fn into_owned (self) -> ClientExtensionsInput < 'static > { let Self { transport_parameters , protocols , } = self ; ClientExtensionsInput { transport_parameters : transport_parameters . map (| x | x . into_owned ()) , protocols , } } }
};
}
