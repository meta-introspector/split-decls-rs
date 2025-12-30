// Generated macro for impl_39 (impl)
macro_rules! Depcrateimpl_39 {
() => {
// Module: crate
// Provides: {"impl_39"}
// Dependencies: {}
impl < 'a > AsSerde < 'a > for tracing_core :: span :: Attributes < 'a > { type Serializable = SerializeAttributes < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeAttributes (self) } }
};
}
