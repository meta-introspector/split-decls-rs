macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < A , T > BorrowMut < [A] > for InlineArray < A , T > { fn borrow_mut (& mut self) -> & mut [A] { self . deref_mut () } }
    };
}

impl_20!()