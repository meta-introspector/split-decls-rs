macro_rules! deps {
    () => {
        ForeignModule!();
    };
}

macro_rules! impl_338 {
    () => {
        deps!();
        impl ForeignModuleDef { pub fn module (& self) -> ForeignModule { with (| cx | cx . foreign_module (* self)) } }
    };
}

impl_338!();