// Generated macro for impl_41 (impl)
macro_rules! Depcrateimpl_41 {
() => {
// Module: crate
// Provides: {"impl_41"}
// Dependencies: {}
impl < 'a > AsSerde < 'a > for tracing_core :: span :: Record < 'a > { type Serializable = SerializeRecord < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeRecord (self) } }
};
}
