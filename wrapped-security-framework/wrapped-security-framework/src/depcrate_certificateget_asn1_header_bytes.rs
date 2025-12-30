// Generated macro for get_asn1_header_bytes (function)
macro_rules! Depcrate_certificateget_asn1_header_bytes {
() => {
// Module: crate::certificate
// Provides: {"get_asn1_header_bytes"}
// Dependencies: {}
# [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] fn get_asn1_header_bytes (pkt : CFString , ksz : u32) -> Option < & 'static [u8] > { use security_framework_sys :: item :: { kSecAttrKeyTypeECSECPrimeRandom , kSecAttrKeyTypeRSA } ; if pkt == unsafe { CFString :: wrap_under_get_rule (kSecAttrKeyTypeRSA) } && ksz == 2048 { return Some (& RSA_2048_ASN1_HEADER) ; } if pkt == unsafe { CFString :: wrap_under_get_rule (kSecAttrKeyTypeRSA) } && ksz == 4096 { return Some (& RSA_4096_ASN1_HEADER) ; } if pkt == unsafe { CFString :: wrap_under_get_rule (kSecAttrKeyTypeECSECPrimeRandom) } && ksz == 256 { return Some (& EC_DSA_SECP_256_R1_ASN1_HEADER) ; } if pkt == unsafe { CFString :: wrap_under_get_rule (kSecAttrKeyTypeECSECPrimeRandom) } && ksz == 384 { return Some (& EC_DSA_SECP_384_R1_ASN1_HEADER) ; } None }
};
}
