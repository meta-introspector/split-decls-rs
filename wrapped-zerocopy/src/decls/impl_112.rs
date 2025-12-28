macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl ByteSlice for cell :: RefMut < '_ , [u8] > { }
    };
}

impl_112!();