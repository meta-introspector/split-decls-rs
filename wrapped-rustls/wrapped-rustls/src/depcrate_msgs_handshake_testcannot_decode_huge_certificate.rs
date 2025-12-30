// Generated macro for cannot_decode_huge_certificate (function)
macro_rules! Depcrate_msgs_handshake_testcannot_decode_huge_certificate {
() => {
// Module: crate::msgs::handshake_test
// Provides: {"cannot_decode_huge_certificate"}
// Dependencies: {}
# [test] fn cannot_decode_huge_certificate () { let mut buf = [0u8 ; 65 * 1024] ; buf [0] = 0x0b ; buf [1] = 0x01 ; buf [2] = 0x00 ; buf [3] = 0x03 ; buf [4] = 0x01 ; buf [5] = 0x00 ; buf [6] = 0x00 ; buf [7] = 0x00 ; buf [8] = 0xff ; buf [9] = 0xfd ; HandshakeMessagePayload :: read_bytes (& buf [.. 0x10000 + 7]) . unwrap () ; buf [1] = 0x01 ; buf [2] = 0x00 ; buf [3] = 0x04 ; buf [4] = 0x01 ; buf [5] = 0x00 ; buf [6] = 0x01 ; assert_eq ! (HandshakeMessagePayload :: read_bytes (& buf [.. 0x10001 + 7]) . unwrap_err () , InvalidMessage :: CertificatePayloadTooLarge) ; }
};
}
