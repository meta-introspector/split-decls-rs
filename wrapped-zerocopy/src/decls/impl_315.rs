macro_rules! deps {
    () => {
        Invariants!();
        Validity!();
        Alignment!();
        Aliasing!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < A : Aliasing , AA : Alignment , V : Validity > Invariants for (A , AA , V) { type Aliasing = A ; type Alignment = AA ; type Validity = V ; }
    };
}

impl_315!()