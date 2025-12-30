// Generated macro for initial_packet_error_type (function)
macro_rules! Depcrate_quic_routerinitial_packet_error_type {
() => {
// Module: crate::quic::router
// Provides: {"initial_packet_error_type"}
// Dependencies: {}
# [doc = " Categorizes errors that are returned when handling packets which are not"] # [doc = " associated with an established connection. The purpose is to suppress"] # [doc = " logging of 'expected' errors (e.g. junk data sent to the UDP socket) to"] # [doc = " prevent DoS."] fn initial_packet_error_type (e : & io :: Error ,) -> labels :: QuicInvalidInitialPacketError { Some (e) . filter (| e | e . kind () == io :: ErrorKind :: Other) . and_then (io :: Error :: get_ref) . and_then (| e | e . downcast_ref ()) . map_or (labels :: QuicInvalidInitialPacketError :: Unexpected , Clone :: clone ,) }
};
}
