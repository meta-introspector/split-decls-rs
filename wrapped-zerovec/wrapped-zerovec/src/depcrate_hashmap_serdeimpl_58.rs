// Generated macro for impl_58 (impl)
macro_rules! Depcrate_hashmap_serdeimpl_58 {
() => {
// Module: crate::hashmap::serde
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , K , V > Serialize for ZeroHashMap < 'a , K , V > where K : ZeroMapKV < 'a > + Serialize + ? Sized , V : ZeroMapKV < 'a > + Serialize + ? Sized , K :: Container : Serialize , V :: Container : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { (& self . displacements , & self . keys , & self . values) . serialize (serializer) } }
};
}
