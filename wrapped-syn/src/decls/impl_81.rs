macro_rules! impl_81 {
    () => {
        impl From < MetaNameValue > for Meta { fn from (meta : MetaNameValue) -> Meta { Meta :: NameValue (meta) } }
    };
}

impl_81!()