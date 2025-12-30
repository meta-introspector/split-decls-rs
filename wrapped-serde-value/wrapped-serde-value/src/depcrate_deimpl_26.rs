// Generated macro for impl_26 (impl)
macro_rules! Depcrate_deimpl_26 {
() => {
// Module: crate::de
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for Value { fn deserialize < D : de :: Deserializer < 'de > > (d : D) -> Result < Self , D :: Error > { d . deserialize_any (ValueVisitor) } }
};
}
