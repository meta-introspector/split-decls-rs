// Generated macro for impl_54 (impl)
macro_rules! Depcrate_serde_implsimpl_54 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for TextRange { # [allow (clippy :: nonminimal_bool)] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let (start , end) = Deserialize :: deserialize (deserializer) ? ; if ! (start <= end) { return Err (de :: Error :: custom (format ! ("invalid range: {:?}..{:?}" , start , end))) ; } Ok (TextRange :: new (start , end)) } }
};
}
