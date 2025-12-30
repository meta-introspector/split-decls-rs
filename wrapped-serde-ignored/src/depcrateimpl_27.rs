// Generated macro for impl_27 (impl)
macro_rules! Depcrateimpl_27 {
() => {
// Module: crate
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'a , 'de , X , F > DeserializeSeed < 'de > for TrackedSeed < 'a , X , F > where X : DeserializeSeed < 'de > , F : FnMut (Path) , { type Value = X :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < X :: Value , D :: Error > where D : de :: Deserializer < 'de > , { self . seed . deserialize (Deserializer { de : deserializer , callback : self . callback , path : self . path , }) } }
};
}
