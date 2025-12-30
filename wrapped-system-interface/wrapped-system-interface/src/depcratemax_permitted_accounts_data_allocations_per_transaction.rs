// Generated macro for MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION (const)
macro_rules! DepcrateMAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION {
() => {
// Module: crate
// Provides: {"MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION"}
// Dependencies: {}
# [doc = " Maximum permitted size of new allocations per transaction, in bytes."] # [doc = ""] # [doc = " The value was chosen such that at least one max sized account could be created,"] # [doc = " plus some additional resize allocations."] pub const MAX_PERMITTED_ACCOUNTS_DATA_ALLOCATIONS_PER_TRANSACTION : i64 = MAX_PERMITTED_DATA_LENGTH as i64 * 2 ;
};
}
