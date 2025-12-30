// Generated macro for impl_727 (impl)
macro_rules! Depcrate_quic_router_connectorimpl_727 {
() => {
// Module: crate::quic::router::connector
// Provides: {"impl_727"}
// Dependencies: {}
impl < Tx > InitialPacketHandler for ClientConnector < Tx > where Tx : DatagramSocketSend + Send + 'static , { fn update (& mut self , ctx : & mut Context < '_ >) -> io :: Result < () > { ClientConnector :: update (self , ctx) } fn handle_initials (& mut self , incoming : Incoming , hdr : Header < 'static > , _ : & mut quiche :: Config ,) -> io :: Result < Option < NewConnection > > { self . on_incoming (incoming , hdr) } }
};
}
