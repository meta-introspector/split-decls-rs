// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl elliptic_curve :: Curve for Sm2 { # [doc = " 24-byte serialized field elements."] type FieldBytesSize = U32 ; # [doc = " Big integer type used for representing field elements."] type Uint = U256 ; # [doc = " Order of SM2's elliptic curve group (i.e. scalar modulus)."] const ORDER : Odd < U256 > = Odd :: < U256 > :: from_be_hex (ORDER_HEX) ; }
};
}
