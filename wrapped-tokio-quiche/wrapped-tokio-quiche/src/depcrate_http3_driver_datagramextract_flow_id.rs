// Generated macro for extract_flow_id (function)
macro_rules! Depcrate_http3_driver_datagramextract_flow_id {
() => {
// Module: crate::http3::driver::datagram
// Provides: {"extract_flow_id"}
// Dependencies: {}
# [doc = " Extracts the DATAGRAM flow ID proxied over the given `stream_id`,"] # [doc = " or `None` if this is not a proxy request."] pub (crate) fn extract_flow_id (stream_id : u64 , headers : & [h3 :: Header] ,) -> Option < u64 > { let mut method = None ; let mut datagram_flow_id : Option < u64 > = None ; let mut protocol = None ; for header in headers { match header . name () { b":method" => method = Some (header . value ()) , b":protocol" => protocol = Some (header . value ()) , b"datagram-flow-id" => datagram_flow_id = std :: str :: from_utf8 (header . value ()) . ok () . and_then (| v | v . parse () . ok ()) , _ => { } , } ; if method . is_some () && (datagram_flow_id . is_some () || protocol . is_some ()) { break ; } } if method == Some (b"CONNECT-UDP") && datagram_flow_id . is_some () { datagram_flow_id } else if method == Some (b"CONNECT") && protocol . is_some () { Some (stream_id / 4) } else { None } }
};
}
