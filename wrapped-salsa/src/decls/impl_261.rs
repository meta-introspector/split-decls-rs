macro_rules! deps {
    () => {
        OptionalAtomicRevision!();
        Revision!();
    };
}

macro_rules! impl_261 {
    () => {
        deps!();
        impl From < Revision > for OptionalAtomicRevision { fn from (value : Revision) -> Self { Self { data : AtomicUsize :: new (value . as_usize ()) , } } }
    };
}

impl_261!()