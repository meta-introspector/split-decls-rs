// Generated macro for simple_conn_send (function)
macro_rules! Depcrate_quic_router_connectorsimple_conn_send {
() => {
// Module: crate::quic::router::connector
// Provides: {"simple_conn_send"}
// Dependencies: {}
# [doc = " Repeatedly send packets until quiche reports that it's done."] # [doc = ""] # [doc = " This does not have to be efficent, since once a connection is established"] # [doc = " the [`crate::quic::io::worker::IoWorker`] will take over sending and"] # [doc = " receiving."] fn simple_conn_send < Tx : DatagramSocketSend + Send + Sync + 'static > (socket_tx : & MaybeConnectedSocket < Arc < Tx > > , conn : & mut QuicheConnection ,) -> io :: Result < () > { let scid = conn . source_id () . into_owned () ; log :: debug ! ("sending client Initials to peer" ; "scid" => ? scid) ; loop { let scid = scid . clone () ; let mut buf = [0 ; MAX_DATAGRAM_SIZE] ; let send_res = conn . send (& mut buf) ; let socket_clone = socket_tx . clone () ; match send_res { Ok ((n , send_info)) => { tokio :: spawn ({ let buf = buf [0 .. n] . to_vec () ; async move { socket_clone . send_to (& buf , send_info . to) . await . inspect_err (| error | { log :: error ! ("error sending client Initial packets to peer" ; "scid" => ? scid , "peer_addr" => send_info . to , "error" => error . to_string ()) ; }) } }) ; } , Err (quiche :: Error :: Done) => break Ok (()) , Err (error) => { log :: error ! ("error writing packets to quiche's internal buffer" ; "scid" => ? scid , "error" => error . to_string ()) ; break Err (std :: io :: Error :: other (error)) ; } , } } }
};
}
