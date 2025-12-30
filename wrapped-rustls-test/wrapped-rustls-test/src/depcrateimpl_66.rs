// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl MockServerVerifier { pub fn accepts_anything () -> Self { Self { cert_rejection_error : None , .. Default :: default () } } pub fn expects_ocsp_response (response : & [u8]) -> Self { Self { expected_ocsp_response : Some (response . to_vec ()) , .. Default :: default () } } pub fn rejects_certificate (err : Error) -> Self { Self { cert_rejection_error : Some (err) , .. Default :: default () } } pub fn rejects_tls12_signatures (err : Error) -> Self { Self { tls12_signature_error : Some (err) , .. Default :: default () } } pub fn rejects_tls13_signatures (err : Error) -> Self { Self { tls13_signature_error : Some (err) , .. Default :: default () } } pub fn offers_no_signature_schemes () -> Self { Self { signature_schemes : vec ! [] , .. Default :: default () } } pub fn expects_raw_public_keys (provider : & CryptoProvider) -> Self { Self { requires_raw_public_keys : true , raw_public_key_algorithms : Some (provider . signature_verification_algorithms) , .. Default :: default () } } }
};
}
