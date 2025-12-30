// Generated macro for InitialPacketHandler (trait)
macro_rules! Depcrate_quic_routerInitialPacketHandler {
() => {
// Module: crate::quic::router
// Provides: {"InitialPacketHandler"}
// Dependencies: {}
# [doc = " An [`InitialPacketHandler`] handles unknown quic initials and processes"] # [doc = " them; generally accepting new connections (acting as a server), or"] # [doc = " establishing a connection to a server (acting as a client). An"] # [doc = " [`InboundPacketRouter`] holds an instance of this trait and routes"] # [doc = " [`Incoming`] packets to it when it receives initials."] # [doc = ""] # [doc = " The handler produces [`quiche::Connection`]s which are then turned into"] # [doc = " [`QuicConnection`](super::QuicConnection), IoWorker pair."] pub trait InitialPacketHandler { fn update (& mut self , _ctx : & mut Context < '_ >) -> io :: Result < () > { Ok (()) } fn handle_initials (& mut self , incoming : Incoming , hdr : Header < 'static > , quiche_config : & mut quiche :: Config ,) -> io :: Result < Option < NewConnection > > ; }
};
}
