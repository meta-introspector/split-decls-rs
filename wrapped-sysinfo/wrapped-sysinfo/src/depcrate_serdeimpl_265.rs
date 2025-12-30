// Generated macro for impl_265 (impl)
macro_rules! Depcrate_serdeimpl_265 {
() => {
// Module: crate::serde
// Provides: {"impl_265"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: ThreadKind { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let (index , variant) = match * self { Self :: Kernel => (0 , "Kernel") , Self :: Userland => (1 , "Userland") , } ; serializer . serialize_unit_variant ("ThreadKind" , index , variant) } }
};
}
