// Generated macro for PublicKey (type)
macro_rules! DepcratePublicKey {
() => {
// Module: crate
// Provides: {"PublicKey"}
// Dependencies: {}
# [doc = " SM2 public key: wrapper type for an elliptic curve point."] # [cfg (feature = "arithmetic")] pub type PublicKey = elliptic_curve :: PublicKey < Sm2 > ;
};
}
