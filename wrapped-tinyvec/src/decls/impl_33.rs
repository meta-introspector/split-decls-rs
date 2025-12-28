macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < A : Array > BorrowMut < [A :: Item] > for ArrayVec < A > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
    };
}

impl_33!();