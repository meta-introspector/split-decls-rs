macro_rules! deps {
    () => {
        TraitFixer!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx , C > TraitFixer < 'tcx , C > where C : ConfigTrait , { }
    };
}

impl_1!()