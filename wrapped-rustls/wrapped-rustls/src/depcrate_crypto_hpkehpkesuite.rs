// Generated macro for HpkeSuite (struct)
macro_rules! Depcrate_crypto_hpkeHpkeSuite {
() => {
// Module: crate::crypto::hpke
// Provides: {"HpkeSuite"}
// Dependencies: {}
# [doc = " An HPKE suite, specifying a key encapsulation mechanism and a symmetric cipher suite."] # [expect (clippy :: exhaustive_structs)] # [derive (Clone , Copy , Debug , Eq , PartialEq)] pub struct HpkeSuite { # [doc = " The choice of HPKE key encapsulation mechanism."] pub kem : HpkeKem , # [doc = " The choice of HPKE symmetric cipher suite."] # [doc = ""] # [doc = " This combines a choice of authenticated encryption with additional data (AEAD) algorithm"] # [doc = " and a key derivation function (KDF)."] pub sym : HpkeSymmetricCipherSuite , }
};
}
