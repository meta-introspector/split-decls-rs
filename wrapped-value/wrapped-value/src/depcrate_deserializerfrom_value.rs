// Generated macro for from_value (function)
macro_rules! Depcrate_deserializerfrom_value {
() => {
// Module: crate::deserializer
// Provides: {"from_value"}
// Dependencies: {}
# [doc = " Interpret a `ConstValue` as an instance of type `T`."] # [inline] pub fn from_value < T : DeserializeOwned > (value : ConstValue) -> Result < T , DeserializerError > { T :: deserialize (value) }
};
}
