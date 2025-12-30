// Generated macro for impl_36 (impl)
macro_rules! Depcrate_datetimeimpl_36 {
() => {
// Module: crate::datetime
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg (feature = "alloc")] impl serde_core :: ser :: Serialize for Time { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde_core :: ser :: Serializer , { Datetime :: from (* self) . serialize (serializer) } }
};
}
