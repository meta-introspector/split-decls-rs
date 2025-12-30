// Generated macro for impl_413 (impl)
macro_rules! Depcrate_ser_implsimpl_413 {
() => {
// Module: crate::ser::impls
// Provides: {"impl_413"}
// Dependencies: {}
# [cfg (any (feature = "std" , not (no_core_net)))] impl Serialize for net :: Ipv6Addr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { const MAX_LEN : usize = 39 ; debug_assert_eq ! (MAX_LEN , "1001:1002:1003:1004:1005:1006:1007:1008" . len ()) ; serialize_display_bounded_length ! (self , MAX_LEN , serializer) } else { self . octets () . serialize (serializer) } } }
};
}
