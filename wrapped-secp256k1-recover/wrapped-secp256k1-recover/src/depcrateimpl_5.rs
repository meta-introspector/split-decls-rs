// Generated macro for impl_5 (impl)
macro_rules! Depcrateimpl_5 {
() => {
// Module: crate
// Provides: {"impl_5"}
// Dependencies: {}
impl From < Secp256k1RecoverError > for u64 { fn from (v : Secp256k1RecoverError) -> u64 { match v { Secp256k1RecoverError :: InvalidHash => 1 , Secp256k1RecoverError :: InvalidRecoveryId => 2 , Secp256k1RecoverError :: InvalidSignature => 3 , } } }
};
}
