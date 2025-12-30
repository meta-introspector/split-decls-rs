// Generated macro for impl_56 (impl)
macro_rules! Depcrate_deimpl_56 {
() => {
// Module: crate::de
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'de > serde_core :: de :: DeserializeSeed < 'de > for DatetimeOrTable < '_ , 'de > { type Value = () ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : serde_core :: de :: Deserializer < 'de > , { deserializer . deserialize_any (self) } }
};
}
