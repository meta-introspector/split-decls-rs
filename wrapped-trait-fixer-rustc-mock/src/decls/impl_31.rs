macro_rules! deps {
    () => {
        MockMetaItem!();
        MockAttribute!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl MockAttribute { pub fn meta_item_list (self) -> Vec < MockMetaItem > { vec ! [MockMetaItem] } }
    };
}

impl_31!()