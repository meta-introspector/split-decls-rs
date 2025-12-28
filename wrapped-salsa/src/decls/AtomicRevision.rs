macro_rules! AtomicRevision {
    () => {
        # [derive (Debug)] pub (crate) struct AtomicRevision { data : AtomicUsize , }
    };
}

AtomicRevision!();