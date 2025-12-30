// Generated macro for WildcardDnsNameRef (struct)
macro_rules! Depcrate_subject_name_dns_nameWildcardDnsNameRef {
() => {
// Module: crate::subject_name::dns_name
// Provides: {"WildcardDnsNameRef"}
// Dependencies: {}
# [doc = " A reference to a DNS Name presented by a server that may include a wildcard."] # [doc = ""] # [doc = " A `WildcardDnsNameRef` is guaranteed to be syntactically valid. The validity rules"] # [doc = " are specified in [RFC 5280 Section 7.2], except that underscores are also"] # [doc = " allowed."] # [doc = ""] # [doc = " Additionally, while [RFC6125 Section 4.1] says that a wildcard label may be of the form"] # [doc = " `<x>*<y>.<DNSID>`, where `<x>` and/or `<y>` may be empty, we follow a stricter policy common"] # [doc = " to most validation libraries (e.g. NSS) and only accept wildcard labels that are exactly `*`."] # [doc = ""] # [doc = " [RFC 5280 Section 7.2]: https://tools.ietf.org/html/rfc5280#section-7.2"] # [doc = " [RFC 6125 Section 4.1]: https://www.rfc-editor.org/rfc/rfc6125#section-4.1"] # [derive (Clone , Copy , Eq , PartialEq , Hash)] pub (crate) struct WildcardDnsNameRef < 'a > (& 'a [u8]) ;
};
}
