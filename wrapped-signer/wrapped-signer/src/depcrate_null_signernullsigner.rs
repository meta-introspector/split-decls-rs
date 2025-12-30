// Generated macro for NullSigner (struct)
macro_rules! Depcrate_null_signerNullSigner {
() => {
// Module: crate::null_signer
// Provides: {"NullSigner"}
// Dependencies: {}
# [doc = " NullSigner - A `Signer` implementation that always produces `Signature::default()`."] # [doc = " Used as a placeholder for absentee signers whose 'Pubkey` is required to construct"] # [doc = " the transaction"] # [derive (Clone , Debug , Default)] pub struct NullSigner { pubkey : Pubkey , }
};
}
