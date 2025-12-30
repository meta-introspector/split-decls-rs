// Generated macro for impl_185 (impl)
macro_rules! Depcrate_subject_name_dns_nameimpl_185 {
() => {
// Module: crate::subject_name::dns_name
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'a > WildcardDnsNameRef < 'a > { # [doc = " Constructs a `WildcardDnsNameRef` from the given input if the input is a"] # [doc = " syntactically-valid DNS name."] pub (crate) fn try_from_ascii (dns_name : & 'a [u8]) -> Result < Self , InvalidDnsNameError > { if ! is_valid_dns_id (untrusted :: Input :: from (dns_name) , IdRole :: Reference , Wildcards :: Allow ,) { return Err (InvalidDnsNameError) ; } Ok (Self (dns_name)) } # [doc = " Yields a reference to the DNS name as a `&str`."] pub (crate) fn as_str (& self) -> & 'a str { core :: str :: from_utf8 (self . 0) . unwrap () } }
};
}
