macro_rules! impl_80 {
    () => {
        impl From < MetaList > for Meta { fn from (meta : MetaList) -> Meta { Meta :: List (meta) } }
    };
}

impl_80!()