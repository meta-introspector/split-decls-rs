macro_rules! deps {
    () => {
        DefId!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl ! PartialOrd for DefId { }
    };
}

impl_105!();