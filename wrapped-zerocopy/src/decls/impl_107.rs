macro_rules! deps {
    () => {
        SplitByteSlice!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        unsafe impl SplitByteSlice for & mut [u8] { # [inline] unsafe fn split_at_unchecked (self , mid : usize) -> (Self , Self) { use core :: slice :: from_raw_parts_mut ; let l_ptr = self . as_mut_ptr () ; let r_ptr = unsafe { l_ptr . add (mid) } ; let l_len = mid ; # [allow (unstable_name_collisions)] let r_len = unsafe { self . len () . unchecked_sub (mid) } ; unsafe { (from_raw_parts_mut (l_ptr , l_len) , from_raw_parts_mut (r_ptr , r_len)) } } }
    };
}

impl_107!();