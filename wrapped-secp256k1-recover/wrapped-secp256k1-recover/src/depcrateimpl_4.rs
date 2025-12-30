// Generated macro for impl_4 (impl)
macro_rules! Depcrateimpl_4 {
() => {
// Module: crate
// Provides: {"impl_4"}
// Dependencies: {}
impl From < u64 > for Secp256k1RecoverError { fn from (v : u64) -> Secp256k1RecoverError { match v { 1 => Secp256k1RecoverError :: InvalidHash , 2 => Secp256k1RecoverError :: InvalidRecoveryId , 3 => Secp256k1RecoverError :: InvalidSignature , _ => panic ! ("Unsupported Secp256k1RecoverError") , } } }
};
}
