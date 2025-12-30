// Generated macro for Host (enum)
macro_rules! Depcrate_hostHost {
() => {
// Module: crate::host
// Provides: {"Host"}
// Dependencies: {}
# [doc = " The host name of an URL."] # [cfg_attr (feature = "serde" , derive (Deserialize , Serialize))] # [derive (Clone , Debug , Eq , Ord , PartialOrd , Hash)] pub enum Host < S = String > { # [doc = " A DNS domain name, as '.' dot-separated labels."] # [doc = " Non-ASCII labels are encoded in punycode per IDNA if this is the host of"] # [doc = " a special URL, or percent encoded for non-special URLs. Hosts for"] # [doc = " non-special URLs are also called opaque hosts."] Domain (S) , # [doc = " An IPv4 address."] # [doc = " `Url::host_str` returns the serialization of this address,"] # [doc = " as four decimal integers separated by `.` dots."] Ipv4 (Ipv4Addr) , # [doc = " An IPv6 address."] # [doc = " `Url::host_str` returns the serialization of that address between `[` and `]` brackets,"] # [doc = " in the format per [RFC 5952 *A Recommendation"] # [doc = " for IPv6 Address Text Representation*](https://tools.ietf.org/html/rfc5952):"] # [doc = " lowercase hexadecimal with maximal `::` compression."] Ipv6 (Ipv6Addr) , }
};
}
