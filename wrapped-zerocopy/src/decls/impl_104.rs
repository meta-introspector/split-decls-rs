macro_rules! deps {
    () => {
        SplitByteSlice!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        unsafe impl SplitByteSlice for & [u8] { # [inline] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) { unsafe { (< [u8] > :: get_unchecked (self , .. mid) , < [u8] > :: get_unchecked (self , mid ..)) } } }
    };
}

impl_104!()