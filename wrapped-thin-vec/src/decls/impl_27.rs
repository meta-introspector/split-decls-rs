macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl < T > BorrowMut < [T] > for ThinVec < T > { fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_27!()