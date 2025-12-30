// Generated macro for impl_363 (impl)
macro_rules! Depcrate_metrics_labelsimpl_363 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_363"}
// Dependencies: {}
impl Serialize for H3Error { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let code = self . 0 ; let v = if code > 0x21 && (code - 0x21) % 0x1f == 0 { "H3_GREASE" } else { match code { 0x33 => "H3_DATAGRAM_ERROR" , 0x100 => "H3_NO_ERROR" , 0x101 => "H3_GENERAL_PROTOCOL_ERROR" , 0x102 => "H3_INTERNAL_ERROR" , 0x103 => "H3_STREAM_CREATION_ERROR" , 0x104 => "H3_CLOSED_CRITICAL_STREAM" , 0x105 => "H3_FRAME_UNEXPECTED" , 0x106 => "H3_FRAME_ERROR" , 0x107 => "H3_EXCESSIVE_LOAD" , 0x108 => "H3_ID_ERROR" , 0x109 => "H3_SETTINGS_ERROR" , 0x10a => "H3_MISSING_SETTINGS" , 0x10b => "H3_REQUEST_REJECTED" , 0x10c => "H3_REQUEST_CANCELLED" , 0x10d => "H3_REQUEST_INCOMPLETE" , 0x10e => "H3_MESSAGE_ERROR" , 0x10f => "H3_CONNECT_ERROR" , 0x110 => "H3_VERSION_FALLBACK" , 0x200 => "QPACK_DECOMPRESSION_FAILED" , 0x201 => "QPACK_ENCODER_STREAM_ERROR" , 0x202 => "QPACK_DECODER_STREAM_ERROR" , _ => "H3_UNKNOWN" , } } ; serializer . serialize_str (v) } }
};
}
