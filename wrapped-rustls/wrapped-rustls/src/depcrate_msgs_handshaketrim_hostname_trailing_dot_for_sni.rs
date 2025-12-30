// Generated macro for trim_hostname_trailing_dot_for_sni (function)
macro_rules! Depcrate_msgs_handshaketrim_hostname_trailing_dot_for_sni {
() => {
// Module: crate::msgs::handshake
// Provides: {"trim_hostname_trailing_dot_for_sni"}
// Dependencies: {}
fn trim_hostname_trailing_dot_for_sni (dns_name : & DnsName < '_ >) -> DnsName < 'static > { let dns_name_str = dns_name . as_ref () ; if dns_name_str . ends_with ('.') { let trimmed = & dns_name_str [0 .. dns_name_str . len () - 1] ; DnsName :: try_from (trimmed) . unwrap () . to_owned () } else { dns_name . to_owned () } }
};
}
