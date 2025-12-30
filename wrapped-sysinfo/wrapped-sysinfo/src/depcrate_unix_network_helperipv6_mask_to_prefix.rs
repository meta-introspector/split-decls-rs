// Generated macro for ipv6_mask_to_prefix (function)
macro_rules! Depcrate_unix_network_helperipv6_mask_to_prefix {
() => {
// Module: crate::unix::network_helper
// Provides: {"ipv6_mask_to_prefix"}
// Dependencies: {}
pub (crate) fn ipv6_mask_to_prefix (mask : Ipv6Addr) -> Result < u8 , & 'static str > { let mask = mask . segments () ; let mut mask_iter = mask . iter () ; let mut prefix = 0 ; for & segment in & mut mask_iter { if segment == 0xffff { prefix += 16 ; } else if segment == 0 { break ; } else { let prefix_bits = (! segment) . leading_zeros () as u8 ; if segment << prefix_bits != 0 { return Err ("invalid ipv6 prefix") ; } prefix += prefix_bits ; break ; } } for & segment in mask_iter { if segment != 0 { return Err ("invalid ipv6 prefix") ; } } Ok (prefix) }
};
}
