// Generated macro for impl_125 (impl)
macro_rules! Depcrateimpl_125 {
() => {
// Module: crate
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Name { fn deserialize < D : Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (Self (String :: deserialize (deserializer) ? . into_boxed_str () . into () ,)) } }
};
}
