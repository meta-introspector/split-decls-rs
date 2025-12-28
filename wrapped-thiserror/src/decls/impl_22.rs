macro_rules! deps {
    () => {
        Sealed!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl Sealed for Path { }
    };
}

impl_22!();