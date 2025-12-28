macro_rules! impl_240 {
    () => {
        impl From < Index > for Member { fn from (index : Index) -> Member { Member :: Unnamed (index) } }
    };
}

impl_240!()