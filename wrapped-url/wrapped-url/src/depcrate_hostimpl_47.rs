// Generated macro for impl_47 (impl)
macro_rules! Depcrate_hostimpl_47 {
() => {
// Module: crate::host
// Provides: {"impl_47"}
// Dependencies: {}
impl < S , T > PartialEq < Host < T > > for Host < S > where S : PartialEq < T > , { fn eq (& self , other : & Host < T >) -> bool { match (self , other) { (Self :: Domain (a) , Host :: Domain (b)) => a == b , (Self :: Ipv4 (a) , Host :: Ipv4 (b)) => a == b , (Self :: Ipv6 (a) , Host :: Ipv6 (b)) => a == b , (_ , _) => false , } } }
};
}
