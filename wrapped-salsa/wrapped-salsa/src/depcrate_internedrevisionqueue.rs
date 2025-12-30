// Generated macro for RevisionQueue (struct)
macro_rules! Depcrate_internedRevisionQueue {
() => {
// Module: crate::interned
// Provides: {"RevisionQueue"}
// Dependencies: {}
# [doc = " Keep track of revisions in which interned values were read, to determine staleness."] # [doc = ""] # [doc = " An interned value is considered stale if it has not been read in the past `REVS`"] # [doc = " revisions. However, we only consider revisions in which interned values were actually"] # [doc = " read, as revisions may be created in bursts."] struct RevisionQueue < C > { lock : Mutex < () > , revisions : Box < [AtomicRevision] > , _configuration : PhantomData < fn () -> C > , }
};
}
