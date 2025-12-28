macro_rules! OptionalAtomicRevision {
    () => {
        # [derive (Debug)] pub (crate) struct OptionalAtomicRevision { data : AtomicUsize , }
    };
}

OptionalAtomicRevision!()