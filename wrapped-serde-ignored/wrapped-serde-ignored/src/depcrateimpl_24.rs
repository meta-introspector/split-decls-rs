// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , 'de , X > de :: EnumAccess < 'de > for CaptureKey < 'a , X > where X : de :: EnumAccess < 'de > , { type Error = X :: Error ; type Variant = X :: Variant ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self :: Variant) , X :: Error > where V : DeserializeSeed < 'de > , { self . delegate . variant_seed (CaptureKey :: new (seed , self . key)) } }
};
}
