macro_rules! deps {
    () => {
        MockTraitFixer!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx > MockTraitFixer < 'tcx > { }
    };
}

impl_1!()