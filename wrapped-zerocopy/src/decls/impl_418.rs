macro_rules! deps {
    () => {
        Unaligned!();
        Unalign!();
    };
}

macro_rules! impl_418 {
    () => {
        deps!();
        impl < T : Unaligned + Eq > Eq for Unalign < T > { }
    };
}

impl_418!()