// Generated macro for serialize (function)
macro_rules! Depcrate_to_serializeserialize {
() => {
// Module: crate::to_serialize
// Provides: {"serialize"}
// Dependencies: {}
# [doc = "\nSerialize an [`sval::Value`] into a [`serde_core::Serializer`].\n"] pub fn serialize < S : serde_core :: Serializer > (serializer : S , value : impl sval :: Value ,) -> Result < S :: Ok , S :: Error > { ToSerialize :: new (value) . serialize (serializer) }
};
}
