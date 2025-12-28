macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl < A : Array > Deref for ArrayVec < A > { type Target = [A :: Item] ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . data . as_slice () [.. self . len as usize] } }
    };
}

impl_10!()