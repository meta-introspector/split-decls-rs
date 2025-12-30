// Generated macro for impl_63 (impl)
macro_rules! Depcrate_certificateimpl_63 {
() => {
// Module: crate::certificate
// Provides: {"impl_63"}
// Dependencies: {}
# [cfg (feature = "pem")] impl < P : Profile > CertificateInner < P > { # [doc = " Parse a chain of pem-encoded certificates from a slice."] # [doc = ""] # [doc = " Returns the list of certificates."] pub fn load_pem_chain (mut input : & [u8]) -> Result < Vec < Self > , der :: Error > { fn find_boundary < T > (haystack : & [T] , needle : & [T]) -> Option < usize > where for < 'a > & 'a [T] : PartialEq , { haystack . windows (needle . len ()) . position (| window | window == needle) } let mut certs = Vec :: new () ; let mut position : usize = 0 ; let end_boundary = & b"-----END CERTIFICATE-----" [..] ; loop { if input . is_empty () { break ; } let last_pos = input . len () - 1 ; match input . get (last_pos) { Some (b'\r') | Some (b'\n') => { input = & input [.. last_pos] ; } _ => break , } } while position + 1 < input . len () { let rest = & input [position ..] ; let end_pos = find_boundary (rest , end_boundary) . ok_or (pem :: Error :: PostEncapsulationBoundary) ? + end_boundary . len () ; let cert_buf = & rest [.. end_pos] ; let cert = Self :: from_pem (cert_buf) ? ; certs . push (cert) ; position += end_pos ; } Ok (certs) } }
};
}
