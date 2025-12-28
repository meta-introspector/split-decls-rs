macro_rules! deps {
    () => {
        Config!();
        UniqueIter!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < T , C : cfg :: Config > FusedIterator for UniqueIter < '_ , T , C > { }
    };
}

impl_72!()