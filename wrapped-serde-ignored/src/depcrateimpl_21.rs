// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
# [doc = " Forwarding impl."] impl < 'a , 'de , X > DeserializeSeed < 'de > for CaptureKey < 'a , X > where X : DeserializeSeed < 'de > , { type Value = X :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < X :: Value , D :: Error > where D : de :: Deserializer < 'de > , { self . delegate . deserialize (CaptureKey :: new (deserializer , self . key)) } }
};
}
