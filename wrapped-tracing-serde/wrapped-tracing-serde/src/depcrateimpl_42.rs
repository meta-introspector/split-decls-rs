// Generated macro for impl_42 (impl)
macro_rules! Depcrateimpl_42 {
() => {
// Module: crate
// Provides: {"impl_42"}
// Dependencies: {}
impl < 'a > AsSerde < 'a > for Level { type Serializable = SerializeLevel < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeLevel (self) } }
};
}
