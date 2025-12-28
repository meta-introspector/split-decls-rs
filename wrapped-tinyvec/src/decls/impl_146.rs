macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < A : Array > BorrowMut < [A :: Item] > for TinyVec < A > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [A :: Item] { & mut * self } }
    };
}

impl_146!();