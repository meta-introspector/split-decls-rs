// Generated macro for Inner (struct)
macro_rules! Depcrate_threadInner {
() => {
// Module: crate::thread
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " The internal representation of a `Thread` handle"] # [doc = ""] # [doc = " We explicitly set the alignment for our guarantee in Thread::into_raw. This"] # [doc = " allows applications to stuff extra metadata bits into the alignment, which"] # [doc = " can be rather useful when working with atomics."] # [repr (align (8))] struct Inner { name : Option < ThreadNameString > , id : ThreadId , parker : Parker , }
};
}
