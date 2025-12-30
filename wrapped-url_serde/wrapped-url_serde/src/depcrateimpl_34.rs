// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'de , T > Serialize for Serde < T > where De < T > : Deserialize < 'de > , for < 'a > Ser < 'a , T > : Serialize { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer { Ser (& self . 0) . serialize (serializer) } }
};
}
