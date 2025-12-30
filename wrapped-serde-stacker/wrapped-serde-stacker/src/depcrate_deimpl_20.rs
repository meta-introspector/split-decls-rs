// Generated macro for impl_20 (impl)
macro_rules! Depcrate_deimpl_20 {
() => {
// Module: crate::de
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'de , S > de :: DeserializeSeed < 'de > for DeserializeSeed < S > where S : de :: DeserializeSeed < 'de > , { type Value = S :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : de :: Deserializer < 'de > , { self . delegate . deserialize (Deserializer { de : deserializer , red_zone : self . param . red_zone , stack_size : self . param . stack_size , }) } }
};
}
