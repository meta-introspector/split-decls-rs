macro_rules! deps {
    () => {
        AtomicRevision!();
        Revision!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl AtomicRevision { pub (crate) const fn start () -> Self { Self { data : AtomicUsize :: new (START) , } } pub (crate) fn load (& self) -> Revision { Revision { generation : unsafe { NonZeroUsize :: new_unchecked (self . data . load (Ordering :: Acquire)) } , } } pub (crate) fn store (& self , r : Revision) { self . data . store (r . as_usize () , Ordering :: Release) ; } }
    };
}

impl_257!()