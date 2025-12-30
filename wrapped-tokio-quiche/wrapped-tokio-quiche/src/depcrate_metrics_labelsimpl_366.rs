// Generated macro for impl_366 (impl)
macro_rules! Depcrate_metrics_labelsimpl_366 {
() => {
// Module: crate::metrics::labels
// Provides: {"impl_366"}
// Dependencies: {}
impl Serialize for QuicError { fn serialize < S : Serializer > (& self , serializer : S) -> Result < S :: Ok , S :: Error > { let v = match self . 0 { 0x0 => "NO_ERROR" , 0x1 => "INTERNAL_ERROR" , 0x2 => "CONNECTION_REFUSED" , 0x3 => "FLOW_CONTROL_ERROR" , 0x4 => "STREAM_LIMIT_ERROR" , 0x5 => "STREAM_STATE_ERROR" , 0x6 => "FINAL_SIZE_ERROR" , 0x7 => "FRAME_ENCODING_ERROR" , 0x8 => "TRANSPORT_PARAMETER_ERROR" , 0x9 => "CONNECTION_ID_LIMIT_ERROR" , 0xa => "PROTOCOL_VIOLATION" , 0xb => "INVALID_TOKEN" , 0xc => "APPLICATION_ERROR" , 0xd => "CRYPTO_BUFFER_EXCEEDED" , 0xe => "KEY_UPDATE_ERROR" , 0xf => "AEAD_LIMIT_REACHED" , 0x10 => "NO_VIABLE_PATH" , 0x11 => "VERSION_NEGOTIATION_ERROR" , 0x100 ..= 0x1ff => "CRYPTO_ERROR" , _ => "QUIC_UNKNOWN" , } ; serializer . serialize_str (v) } }
};
}
