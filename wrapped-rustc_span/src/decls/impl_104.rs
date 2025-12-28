macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl ! Ord for DefId { }
    };
}

impl_104!()