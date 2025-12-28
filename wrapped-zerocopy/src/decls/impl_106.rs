macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl ByteSlice for & mut [u8] { }
    };
}

impl_106!()