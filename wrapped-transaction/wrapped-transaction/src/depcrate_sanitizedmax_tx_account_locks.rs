// Generated macro for MAX_TX_ACCOUNT_LOCKS (const)
macro_rules! Depcrate_sanitizedMAX_TX_ACCOUNT_LOCKS {
() => {
// Module: crate::sanitized
// Provides: {"MAX_TX_ACCOUNT_LOCKS"}
// Dependencies: {}
# [doc = " Maximum number of accounts that a transaction may lock."] # [doc = " 128 was chosen because it is the minimum number of accounts"] # [doc = " needed for the Neon EVM implementation."] pub const MAX_TX_ACCOUNT_LOCKS : usize = 128 ;
};
}
