macro_rules! deps {
    () => {
        ByteSliceMut!();
        ByteSlice!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < B : ByteSlice + DerefMut > ByteSliceMut for B { }
    };
}

impl_93!()