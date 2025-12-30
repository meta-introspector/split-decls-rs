// Generated macro for impl_40 (impl)
macro_rules! Depcrateimpl_40 {
() => {
// Module: crate
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > AsSerde < 'a > for tracing_core :: span :: Id { type Serializable = SerializeId < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeId (self) } }
};
}
