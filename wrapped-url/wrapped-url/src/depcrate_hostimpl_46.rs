// Generated macro for impl_46 (impl)
macro_rules! Depcrate_hostimpl_46 {
() => {
// Module: crate::host
// Provides: {"impl_46"}
// Dependencies: {}
impl < S : AsRef < str > > fmt :: Display for Host < S > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { match * self { Self :: Domain (ref domain) => domain . as_ref () . fmt (f) , Self :: Ipv4 (ref addr) => addr . fmt (f) , Self :: Ipv6 (ref addr) => { f . write_str ("[") ? ; write_ipv6 (addr , f) ? ; f . write_str ("]") } } } }
};
}
