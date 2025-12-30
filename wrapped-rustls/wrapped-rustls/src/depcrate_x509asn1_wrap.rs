// Generated macro for asn1_wrap (function)
macro_rules! Depcrate_x509asn1_wrap {
() => {
// Module: crate::x509
// Provides: {"asn1_wrap"}
// Dependencies: {}
fn asn1_wrap (tag : u8 , bytes_a : & [u8] , bytes_b : & [u8]) -> Vec < u8 > { let len = bytes_a . len () + bytes_b . len () ; if len <= 0x7f { let mut ret = Vec :: with_capacity (2 + len) ; ret . push (tag) ; ret . push (len as u8) ; ret . extend_from_slice (bytes_a) ; ret . extend_from_slice (bytes_b) ; ret } else { let size = len . to_be_bytes () ; let leading_zero_bytes = size . iter () . position (| & x | x != 0) . unwrap_or (size . len ()) ; assert ! (leading_zero_bytes < size . len ()) ; let encoded_bytes = size . len () - leading_zero_bytes ; let mut ret = Vec :: with_capacity (2 + encoded_bytes + len) ; ret . push (tag) ; ret . push (0x80 + encoded_bytes as u8) ; ret . extend_from_slice (& size [leading_zero_bytes ..]) ; ret . extend_from_slice (bytes_a) ; ret . extend_from_slice (bytes_b) ; ret } }
};
}
