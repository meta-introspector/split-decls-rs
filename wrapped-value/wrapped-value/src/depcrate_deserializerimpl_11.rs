// Generated macro for impl_11 (impl)
macro_rules! Depcrate_deserializerimpl_11 {
() => {
// Module: crate::deserializer
// Provides: {"impl_11"}
// Dependencies: {}
impl ConstValue { # [inline] fn unexpected (& self) -> Unexpected { match self { ConstValue :: Null => Unexpected :: Unit , ConstValue :: Number (_) => Unexpected :: Other ("number") , ConstValue :: String (v) => Unexpected :: Str (v) , ConstValue :: Boolean (v) => Unexpected :: Bool (* v) , ConstValue :: Binary (v) => Unexpected :: Bytes (v) , ConstValue :: Enum (v) => Unexpected :: Str (v) , ConstValue :: List (_) => Unexpected :: Seq , ConstValue :: Object (_) => Unexpected :: Map , } } }
};
}
