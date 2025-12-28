macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl < A : Array > DerefMut for ArrayVec < A > { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . data . as_slice_mut () [.. self . len as usize] } }
    };
}

impl_11!()