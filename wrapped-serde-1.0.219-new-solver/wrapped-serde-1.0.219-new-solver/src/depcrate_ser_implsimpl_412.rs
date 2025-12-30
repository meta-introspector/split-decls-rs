// Generated macro for impl_412 (impl)
macro_rules! Depcrate_ser_implsimpl_412 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_412"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl Serialize for net :: Ipv4Addr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { const MAX_LEN : usize = 15 ; debug_assert_eq ! (MAX_LEN , "101.102.103.104" . len ()) ; let mut buf = [b'.' ; MAX_LEN] ; let mut written = format_u8 (self . octets () [0] , & mut buf) ; for oct in & self . octets () [1 ..] { written += format_u8 (* oct , & mut buf [written + 1 ..]) + 1 ; } let buf = unsafe { str :: from_utf8_unchecked (& buf [.. written]) } ; serializer . serialize_str (buf) } else { self . octets () . serialize (serializer) } } }
};
}
