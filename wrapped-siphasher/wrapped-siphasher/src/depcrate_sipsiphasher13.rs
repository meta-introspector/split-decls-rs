// Generated macro for SipHasher13 (struct)
macro_rules! Depcrate_sipSipHasher13 {
() => {
// Module: crate::sip
// Provides: {"SipHasher13"}
// Dependencies: {}
# [doc = " An implementation of SipHash 1-3."] # [doc = ""] # [doc = " See: <https://www.aumasson.jp/siphash/siphash.pdf>"] # [derive (Debug , Clone , Copy , Default)] # [cfg_attr (feature = "serde" , derive (serde :: Serialize , serde :: Deserialize))] pub struct SipHasher13 { hasher : Hasher < Sip13Rounds > , }
};
}
