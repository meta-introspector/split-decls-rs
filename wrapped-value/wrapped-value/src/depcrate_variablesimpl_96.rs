// Generated macro for impl_96 (impl)
macro_rules! Depcrate_variablesimpl_96 {
() => {
// Module: crate::variables
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Variables { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self (< Option < BTreeMap < Name , ConstValue > > > :: deserialize (deserializer) ? . unwrap_or_default () ,)) } }
};
}
