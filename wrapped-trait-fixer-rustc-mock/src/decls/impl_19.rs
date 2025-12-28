macro_rules! deps {
    () => {
        MockLangItems!();
        DefId!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl MockLangItems { pub fn clone_trait (self) -> Option < DefId > { Some (DefId) } }
    };
}

impl_19!();