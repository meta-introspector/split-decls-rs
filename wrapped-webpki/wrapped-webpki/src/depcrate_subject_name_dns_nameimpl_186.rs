// Generated macro for impl_186 (impl)
macro_rules! Depcrate_subject_name_dns_nameimpl_186 {
() => {
// Module: crate::subject_name::dns_name
// Provides: {"impl_186"}
// Dependencies: {}
impl core :: fmt :: Debug for WildcardDnsNameRef < '_ > { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> Result < () , core :: fmt :: Error > { f . write_str ("WildcardDnsNameRef(\"") ? ; for & ch in self . 0 { f . write_char (char :: from (ch) . to_ascii_lowercase ()) ? ; } f . write_str ("\")") } }
};
}
