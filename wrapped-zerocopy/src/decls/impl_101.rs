macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl ByteSlice for & [u8] { }
    };
}

impl_101!();