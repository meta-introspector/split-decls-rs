macro_rules! deps {
    () => {
        Revision!();
        AtomicRevision!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl From < Revision > for AtomicRevision { fn from (value : Revision) -> Self { Self { data : AtomicUsize :: new (value . as_usize ()) , } } }
    };
}

impl_256!();