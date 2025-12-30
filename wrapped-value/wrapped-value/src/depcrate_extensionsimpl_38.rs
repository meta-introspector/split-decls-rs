// Generated macro for impl_38 (impl)
macro_rules! Depcrate_extensionsimpl_38 {
() => {
// Module: crate::extensions
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Extensions { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self (< Option < HashMap < _ , _ > > > :: deserialize (deserializer) ? . unwrap_or_default () ,)) } }
};
}
