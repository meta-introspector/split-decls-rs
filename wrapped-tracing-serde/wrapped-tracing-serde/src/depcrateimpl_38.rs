// Generated macro for impl_38 (impl)
macro_rules! Depcrateimpl_38 {
() => {
// Module: crate
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a > AsSerde < 'a > for tracing_core :: Event < 'a > { type Serializable = SerializeEvent < 'a > ; fn as_serde (& 'a self) -> Self :: Serializable { SerializeEvent (self) } }
};
}
