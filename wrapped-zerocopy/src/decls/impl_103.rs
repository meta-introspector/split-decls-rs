macro_rules! deps {
    () => {
        CloneableByteSlice!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl CloneableByteSlice for & [u8] { }
    };
}

impl_103!();