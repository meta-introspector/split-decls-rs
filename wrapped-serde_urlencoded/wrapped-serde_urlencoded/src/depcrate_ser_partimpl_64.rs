// Generated macro for impl_64 (impl)
macro_rules! Depcrate_ser_partimpl_64 {
() => {
// Module: crate::ser::part
// Provides: {"impl_64"}
// Dependencies: {}
impl < S : Sink > PartSerializer < S > { fn serialize_integer < I > (self , value : I) -> Result < S :: Ok , Error > where I : itoa :: Integer , { let mut buf = itoa :: Buffer :: new () ; let part = buf . format (value) ; ser :: Serializer :: serialize_str (self , part) } fn serialize_floating < F > (self , value : F) -> Result < S :: Ok , Error > where F : ryu :: Float , { let mut buf = ryu :: Buffer :: new () ; let part = buf . format (value) ; ser :: Serializer :: serialize_str (self , part) } }
};
}
