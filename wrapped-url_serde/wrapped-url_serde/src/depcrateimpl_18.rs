// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'a , String > Serialize for Ser < 'a , Host < String > > where String : AsRef < str > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { match * self . 0 { Host :: Domain (ref s) => serializer . serialize_str (s . as_ref ()) , Host :: Ipv4 (_) | Host :: Ipv6 (_) => { const MAX_LEN : usize = 47 ; let mut buffer = [0 ; MAX_LEN] ; serializer . serialize_str (display_into_buffer (& self . 0 , & mut buffer)) } } } }
};
}
