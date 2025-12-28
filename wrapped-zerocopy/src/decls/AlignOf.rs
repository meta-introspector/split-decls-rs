macro_rules! AlignOf {
    () => {
        # [doc = " A type whose size is equal to `align_of::<T>()`."] # [repr (C)] pub struct AlignOf < T > { _u : u8 , _a : [T ; 0] , }
    };
}

AlignOf!();