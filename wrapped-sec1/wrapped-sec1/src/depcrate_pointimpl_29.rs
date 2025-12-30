// Generated macro for impl_29 (impl)
macro_rules! Depcrate_pointimpl_29 {
() => {
// Module: crate::point
// Provides: {"impl_29"}
// Dependencies: {}
impl < Size > fmt :: UpperHex for EncodedPoint < Size > where Size : ModulusSize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:X}" , HexDisplay (self . as_bytes ())) } }
};
}
