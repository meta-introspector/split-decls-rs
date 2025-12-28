macro_rules! impl_248 {
    () => {
        impl From < usize > for Index { fn from (index : usize) -> Index { assert ! (index < u32 :: MAX as usize) ; Index { index : index as u32 , span : Span :: call_site () , } } }
    };
}

impl_248!()