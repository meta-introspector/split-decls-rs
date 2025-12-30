// Generated macro for SipHasher24 (struct)
macro_rules! Depcrate_sipSipHasher24 {
() => {
// Module: crate::sip
// Provides: {"SipHasher24"}
// Dependencies: {}
# [doc = " An implementation of SipHash 2-4."] # [doc = ""] # [doc = " See: <https://www.aumasson.jp/siphash/siphash.pdf>"] # [derive (Debug , Clone , Copy , Default)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct SipHasher24 { hasher : Hasher < Sip24Rounds > , }
};
}
