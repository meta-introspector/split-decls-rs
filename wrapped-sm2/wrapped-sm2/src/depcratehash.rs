// Generated macro for Hash (type)
macro_rules! DepcrateHash {
() => {
// Module: crate
// Provides: {"Hash"}
// Dependencies: {}
# [doc = " SM3 hash output."] # [cfg (feature = "dsa")] type Hash = sm3 :: digest :: Output < sm3 :: Sm3 > ;
};
}
