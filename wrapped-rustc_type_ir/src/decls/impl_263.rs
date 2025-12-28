macro_rules! deps {
    () => {
        Interner!();
        Canonical!();
    };
}

macro_rules! impl_263 {
    () => {
        deps!();
        impl < I : Interner , V : Eq > Eq for Canonical < I , V > { }
    };
}

impl_263!();