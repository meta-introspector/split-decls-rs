// Generated macro for impl_44 (impl)
macro_rules! Depcrate_providerimpl_44 {
() => {
// Module: crate::provider
// Provides: {"impl_44"}
// Dependencies: {}
# [cfg (all (feature = "alloc" , feature = "serde"))] impl serde :: Serialize for VariantOffsets { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { use alloc :: fmt :: Write ; let mut r = alloc :: format ! ("{:+02}:{:02}" , self . standard . hours_part () , self . standard . minutes_part () ,) ; if self . standard . seconds_part () != 0 { let _infallible = write ! (& mut r , ":{:02}" , self . standard . seconds_part ()) ; } if let Some (dst) = self . daylight { let _infallible = write ! (& mut r , "/{:+02}:{:02}" , dst . hours_part () , dst . minutes_part () ,) ; if dst . seconds_part () != 0 { let _infallible = write ! (& mut r , ":{:02}" , dst . seconds_part ()) ; } } serializer . serialize_str (& r) } else { self . to_unaligned () . serialize (serializer) } } }
};
}
