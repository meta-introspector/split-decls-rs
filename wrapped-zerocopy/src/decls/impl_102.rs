macro_rules! deps {
    () => {
        CopyableByteSlice!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl CopyableByteSlice for & [u8] { }
    };
}

impl_102!()