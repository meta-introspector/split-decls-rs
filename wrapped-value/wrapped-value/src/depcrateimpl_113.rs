// Generated macro for impl_113 (impl)
macro_rules! Depcrateimpl_113 {
() => {
// Module: crate
// Provides: {"impl_113"}
// Dependencies: {}
impl Serialize for Name { fn serialize < S : Serializer > (& self , serializer : S) -> std :: result :: Result < S :: Ok , S :: Error > { serializer . serialize_str (& self . 0) } }
};
}
