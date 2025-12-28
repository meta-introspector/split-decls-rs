macro_rules! deps {
    () => {
        BlockedOnInner!();
        BlockOnTransferredOwner!();
        Running!();
        BlockResult!();
        Cycle!();
    };
}

macro_rules! impl_271 {
    () => {
        deps!();
        impl < 'me > BlockOnTransferredOwner < 'me > { # [doc = " Block on the other thread to complete the computation."] pub (super) fn block (self , query_mutex_guard : SyncGuard < 'me >) -> BlockResult < 'me > { if self . thread_id == self . other_id { return BlockResult :: Cycle ; } if self . dg . depends_on (self . other_id , self . thread_id) { crate :: tracing :: debug ! ("block_on: cycle detected for {:?} in thread {thread_id:?} on {:?}" , self . database_key , self . other_id , thread_id = self . thread_id) ; return BlockResult :: Cycle ; } BlockResult :: Running (Running (Box :: new (BlockedOnInner { dg : self . dg , query_mutex_guard , database_key : self . database_key , other_id : self . other_id , thread_id : self . thread_id , }))) } }
    };
}

impl_271!()