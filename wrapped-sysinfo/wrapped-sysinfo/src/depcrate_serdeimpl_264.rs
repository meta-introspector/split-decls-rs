// Generated macro for impl_264 (impl)
macro_rules! Depcrate_serdeimpl_264 {
() => {
// Module: crate::serde
// Provides: {"impl_264"}
// Dependencies: {}
# [cfg (feature = "system")] impl Serialize for crate :: CGroupLimits { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_struct ("CGroupLimits" , 3) ? ; state . serialize_field ("total_memory" , & self . total_memory) ? ; state . serialize_field ("free_memory" , & self . free_memory) ? ; state . serialize_field ("free_swap" , & self . free_swap) ? ; state . end () } }
};
}
