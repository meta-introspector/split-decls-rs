// Generated macro for impl_426 (impl)
macro_rules! Depcrate_runtimeimpl_426 {
() => {
// Module: crate::runtime
// Provides: {"impl_426"}
// Dependencies: {}
impl < 'me > BlockOnTransferredOwner < 'me > { # [doc = " Block on the other thread to complete the computation."] pub (super) fn block (self , query_mutex_guard : SyncGuard < 'me >) -> BlockResult < 'me > { if self . thread_id == self . other_id { return BlockResult :: Cycle ; } if self . dg . depends_on (self . other_id , self . thread_id) { crate :: tracing :: debug ! ("block_on: cycle detected for {:?} in thread {thread_id:?} on {:?}" , self . database_key , self . other_id , thread_id = self . thread_id) ; return BlockResult :: Cycle ; } BlockResult :: Running (Running (Box :: new (BlockedOnInner { dg : self . dg , query_mutex_guard , database_key : self . database_key , other_id : self . other_id , thread_id : self . thread_id , }))) } }
};
}
