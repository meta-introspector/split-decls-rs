macro_rules! deps {
    () => {
        DifferenceIter!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < L : Iterator < Item = T > , R : Iterator < Item = T > , T , U : PartialEq , F : Fn (& T) -> U > DifferenceIter < L , R , F > { fn new (left : L , right : R , compare : F) -> Self { Self { left : left . fuse () , right , compare , } } }
    };
}

impl_50!()