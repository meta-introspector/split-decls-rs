// Generated macro for ipv4_mask_to_prefix (function)
macro_rules! Depcrate_unix_network_helperipv4_mask_to_prefix {
() => {
// Module: crate::unix::network_helper
// Provides: {"ipv4_mask_to_prefix"}
// Dependencies: {}
pub (crate) fn ipv4_mask_to_prefix (mask : Ipv4Addr) -> Result < u8 , & 'static str > { let mask = u32 :: from (mask) ; let prefix = (! mask) . leading_zeros () as u8 ; if (u64 :: from (mask) << prefix) & 0xffff_ffff != 0 { Err ("invalid ipv4 prefix") } else { Ok (prefix) } }
};
}
