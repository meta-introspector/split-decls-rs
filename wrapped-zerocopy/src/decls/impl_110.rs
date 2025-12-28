macro_rules! deps {
    () => {
        ByteSlice!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        # [allow (clippy :: undocumented_unsafe_blocks)] unsafe impl ByteSlice for cell :: Ref < '_ , [u8] > { }
    };
}

impl_110!()