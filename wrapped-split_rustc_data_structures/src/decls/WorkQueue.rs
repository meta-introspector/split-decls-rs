macro_rules! WorkQueue {
    () => {
        # [doc = " A work queue is a handy data structure for tracking work left to"] # [doc = " do. (For example, basic blocks left to process.) It is basically a"] # [doc = " de-duplicating queue; so attempting to insert X if X is already"] # [doc = " enqueued has no effect. This implementation assumes that the"] # [doc = " elements are dense indices, so it can allocate the queue to size"] # [doc = " and also use a bit set to track occupancy."] pub struct WorkQueue < T : Idx > { deque : VecDeque < T > , set : DenseBitSet < T > , }
    };
}

WorkQueue!()