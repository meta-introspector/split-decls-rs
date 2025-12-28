macro_rules! deps {
    () => {
        ForeignModule!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl ForeignModule { pub fn items (& self) -> Vec < ForeignDef > { with (| cx | cx . foreign_items (self . def_id)) } }
    };
}

impl_340!()