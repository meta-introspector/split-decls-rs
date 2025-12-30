// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl From < & str > for TrustBits { fn from (value : & str) -> Self { match value { "Websites" => TrustBits :: Websites , "Email" => TrustBits :: Email , "Code" => TrustBits :: Code , "All Trust Bits Turned Off" => TrustBits :: AllTrustBitsTurnedOff , val => panic ! ("unknown trust bit: {val:?}") , } } }
};
}
