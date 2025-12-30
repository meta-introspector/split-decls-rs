// Generated macro for TransactionAccountLocks (struct)
macro_rules! Depcrate_sanitizedTransactionAccountLocks {
() => {
// Module: crate::sanitized
// Provides: {"TransactionAccountLocks"}
// Dependencies: {}
# [doc = " Set of accounts that must be locked for safe transaction processing"] # [derive (Debug , Clone , Default , Eq , PartialEq)] pub struct TransactionAccountLocks < 'a > { # [doc = " List of readonly account key locks"] pub readonly : Vec < & 'a Address > , # [doc = " List of writable account key locks"] pub writable : Vec < & 'a Address > , }
};
}
