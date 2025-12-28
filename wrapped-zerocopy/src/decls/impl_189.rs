macro_rules! deps {
    () => {
        AlignmentError!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < Src : Eq , Dst : ? Sized > Eq for AlignmentError < Src , Dst > { }
    };
}

impl_189!();