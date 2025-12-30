// Generated macro for impl_1220 (impl)
macro_rules! Depcrate_crypto_aws_lc_rs_quicimpl_1220 {
() => {
// Module: crate::crypto::aws_lc_rs::quic
// Provides: {"impl_1220"}
// Dependencies: {}
impl HeaderProtectionKey { pub (crate) fn new (key : AeadKey , alg : & 'static aead :: quic :: Algorithm) -> Self { Self (aead :: quic :: HeaderProtectionKey :: new (alg , key . as_ref ()) . unwrap ()) } fn xor_in_place (& self , sample : & [u8] , first : & mut u8 , packet_number : & mut [u8] , masked : bool ,) -> Result < () , Error > { let mask = self . 0 . new_mask (sample) . map_err (| _ | Error :: ApiMisuse (ApiMisuse :: InvalidQuicHeaderProtectionSampleLength)) ? ; let (first_mask , pn_mask) = mask . split_first () . unwrap () ; if packet_number . len () > pn_mask . len () { return Err (ApiMisuse :: InvalidQuicHeaderProtectionPacketNumberLength . into ()) ; } const LONG_HEADER_FORM : u8 = 0x80 ; let bits = match * first & LONG_HEADER_FORM == LONG_HEADER_FORM { true => 0x0f , false => 0x1f , } ; let first_plain = match masked { true => * first ^ (first_mask & bits) , false => * first , } ; let pn_len = (first_plain & 0x03) as usize + 1 ; * first ^= first_mask & bits ; for (dst , m) in packet_number . iter_mut () . zip (pn_mask) . take (pn_len) { * dst ^= m ; } Ok (()) } }
};
}
