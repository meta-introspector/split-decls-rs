// Generated macro for use_40 (pub_use)
macro_rules! Depcrate_transactionuse_40 {
() => {
// Module: crate::transaction
// Provides: {"use_40"}
// Dependencies: {}
# [deprecated (since = "2.2.0" , note = "Use solana_transaction crate instead")] pub use solana_transaction :: { sanitized :: { MessageHash , SanitizedTransaction , TransactionAccountLocks , MAX_TX_ACCOUNT_LOCKS } , uses_durable_nonce , versioned :: { sanitized :: SanitizedVersionedTransaction , Legacy , TransactionVersion , VersionedTransaction , } , Transaction , TransactionVerificationMode , } ;
};
}
