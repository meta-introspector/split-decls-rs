macro_rules! deps {
    () => {
        SplitByteSlice!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        unsafe impl SplitByteSlice for cell :: Ref < '_ , [u8] > { # [inline] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) { cell :: Ref :: map_split (self , | slice | unsafe { SplitByteSlice :: split_at_unchecked (slice , mid) }) } }
    };
}

impl_111!()