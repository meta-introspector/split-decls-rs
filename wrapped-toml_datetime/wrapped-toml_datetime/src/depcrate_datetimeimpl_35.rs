// Generated macro for impl_35 (impl)
macro_rules! Depcrate_datetimeimpl_35 {
() => {
// Module: crate::datetime
// Provides: {"impl_35"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg (feature = "alloc")] impl serde_core :: ser :: Serialize for Date { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: ser :: Serializer , { Datetime :: from (* self) . serialize (serializer) } }
};
}
