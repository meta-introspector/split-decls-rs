macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl < 's , T > BorrowMut < [T] > for SliceVec < 's , T > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [T] { & mut * self } }
    };
}

impl_97!()