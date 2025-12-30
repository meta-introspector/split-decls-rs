// Generated macro for impl_28 (impl)
macro_rules! Depcrate_pointimpl_28 {
() => {
// Module: crate::point
// Provides: {"impl_28"}
// Dependencies: {}
impl < Size > fmt :: LowerHex for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:x}" , HexDisplay (self . as_bytes ())) } }
};
}
