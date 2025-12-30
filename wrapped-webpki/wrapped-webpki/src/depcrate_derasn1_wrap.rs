// Generated macro for asn1_wrap (function)
macro_rules! Depcrate_derasn1_wrap {
() => {
// Module: crate::der
// Provides: {"asn1_wrap"}
// Dependencies: {}
# [doc = " Prepend `bytes` with the given ASN.1 [`Tag`] and appropriately encoded length byte(s)."] # [doc = " Useful for \"adding back\" ASN.1 bytes to parsed content."] # [cfg (feature = "alloc")] # [allow (clippy :: as_conversions)] pub (crate) fn asn1_wrap (tag : Tag , bytes : & [u8]) -> Vec < u8 > { let len = bytes . len () ; if len < usize :: from (SHORT_FORM_LEN_MAX) { let mut ret = Vec :: with_capacity (2 + len) ; ret . push (tag . into ()) ; ret . push (len as u8) ; ret . extend_from_slice (bytes) ; ret } else { let size = len . to_be_bytes () ; let leading_zero_bytes = size . iter () . position (| & byte | byte != 0) . unwrap_or (size . len ()) ; assert ! (leading_zero_bytes < size . len ()) ; let encoded_bytes = size . len () - leading_zero_bytes ; let mut ret = Vec :: with_capacity (2 + encoded_bytes + len) ; let number_of_length_bytes_byte = SHORT_FORM_LEN_MAX + encoded_bytes as u8 ; ret . push (tag . into ()) ; ret . push (number_of_length_bytes_byte) ; ret . extend_from_slice (& size [leading_zero_bytes ..]) ; ret . extend_from_slice (bytes) ; ret } }
};
}
