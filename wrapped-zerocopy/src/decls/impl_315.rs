macro_rules! deps {
    () => {
        Validity!();
        Aliasing!();
        Alignment!();
        Invariants!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < A : Aliasing , AA : Alignment , V : Validity > Invariants for (A , AA , V) { type Aliasing = A ; type Alignment = AA ; type Validity = V ; }
    };
}

impl_315!();