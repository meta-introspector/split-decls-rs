// Generated macro for impl_146 (impl)
macro_rules! Depcrate_request_builderimpl_146 {
() => {
// Module: crate::request::builder
// Provides: {"impl_146"}
// Dependencies: {}
impl < T > AsyncBuilder for T where T : Builder , { type Output = < T as Builder > :: Output ; fn assemble < S > (self , signature : BitString , signer : & S) -> Result < Self :: Output > where S : Keypair + DynSignatureAlgorithmIdentifier , S :: VerifyingKey : EncodePublicKey , { < T as Builder > :: assemble (self , signature , signer) } fn finalize < S > (& mut self , signer : & S) -> Result < vec :: Vec < u8 > > where S : Keypair + DynSignatureAlgorithmIdentifier , S :: VerifyingKey : EncodePublicKey , { < T as Builder > :: finalize (self , signer) } }
};
}
