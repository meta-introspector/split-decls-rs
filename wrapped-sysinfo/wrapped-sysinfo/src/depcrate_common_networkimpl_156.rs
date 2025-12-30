// Generated macro for impl_156 (impl)
macro_rules! Depcrate_common_networkimpl_156 {
() => {
// Module: crate::common::network
// Provides: {"impl_156"}
// Dependencies: {}
impl FromStr for IpNetwork { type Err = IpNetworkFromStrError ; # [allow (clippy :: from_str_radix_10)] fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut parts = s . split ('/') ; let Some (addr) = parts . next () else { return Err (IpNetworkFromStrError :: InvalidAddrFormat) ; } ; let Some (prefix) = parts . next () else { return Err (IpNetworkFromStrError :: InvalidAddrFormat) ; } ; if parts . next () . is_some () { return Err (IpNetworkFromStrError :: InvalidAddrFormat) ; } Ok (IpNetwork { addr : IpAddr :: from_str (addr) . map_err (IpNetworkFromStrError :: AddrParseError) ? , prefix : u8 :: from_str_radix (prefix , 10) . map_err (IpNetworkFromStrError :: PrefixError) ? , }) } }
};
}
