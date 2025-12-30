// Generated macro for SystemError (enum)
macro_rules! Depcrate_errorSystemError {
() => {
// Module: crate::error
// Provides: {"SystemError"}
// Dependencies: {}
# [cfg_attr (test , derive (strum_macros :: FromRepr , strum_macros :: EnumIter))] # [cfg_attr (feature = "serde" , derive (serde_derive :: Deserialize , serde_derive :: Serialize))] # [derive (Clone , Debug , PartialEq , Eq)] pub enum SystemError { # [doc = " An account with the same address already exists."] AccountAlreadyInUse , # [doc = " Account does not have enough SOL to perform the operation."] ResultWithNegativeLamports , # [doc = " Cannot assign account to this program id."] InvalidProgramId , # [doc = " Cannot allocate account data of this length."] InvalidAccountDataLength , # [doc = " Length of requested seed is too long."] MaxSeedLengthExceeded , # [doc = " Provided address does not match addressed derived from seed."] AddressWithSeedMismatch , # [doc = " Advancing stored nonce requires a populated RecentBlockhashes sysvar."] NonceNoRecentBlockhashes , # [doc = " Stored nonce is still in recent_blockhashes."] NonceBlockhashNotExpired , # [doc = " Specified nonce does not match stored nonce."] NonceUnexpectedBlockhashValue , }
};
}
