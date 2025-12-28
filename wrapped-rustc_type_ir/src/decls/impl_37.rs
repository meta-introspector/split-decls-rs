macro_rules! deps {
    () => {
        Interner!();
        NoSolution!();
        TypeError!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl < I : Interner > From < TypeError < I > > for NoSolution { fn from (_ : TypeError < I >) -> NoSolution { NoSolution } }
    };
}

impl_37!();