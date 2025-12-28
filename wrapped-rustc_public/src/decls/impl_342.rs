macro_rules! deps {
    () => {
        ForeignItemKind!();
    };
}

macro_rules! impl_342 {
    () => {
        deps!();
        impl ForeignDef { pub fn kind (& self) -> ForeignItemKind { with (| cx | cx . foreign_item_kind (* self)) } }
    };
}

impl_342!()