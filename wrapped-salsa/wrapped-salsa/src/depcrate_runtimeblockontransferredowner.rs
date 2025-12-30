// Generated macro for BlockOnTransferredOwner (struct)
macro_rules! Depcrate_runtimeBlockOnTransferredOwner {
() => {
// Module: crate::runtime
// Provides: {"BlockOnTransferredOwner"}
// Dependencies: {}
pub (super) struct BlockOnTransferredOwner < 'me > { dg : crate :: sync :: MutexGuard < 'me , DependencyGraph > , # [doc = " The query that we're trying to claim."] database_key : DatabaseKeyIndex , # [doc = " The thread that currently owns the lock for the transferred query."] other_id : ThreadId , # [doc = " The current thread that is trying to claim the transferred query."] thread_id : ThreadId , }
};
}
