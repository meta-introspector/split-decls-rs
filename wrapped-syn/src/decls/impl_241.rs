macro_rules! impl_241 {
    () => {
        impl From < usize > for Member { fn from (index : usize) -> Member { Member :: Unnamed (Index :: from (index)) } }
    };
}

impl_241!()