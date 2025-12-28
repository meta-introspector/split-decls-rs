macro_rules! deps {
    () => {
        MaybeUninit!();
        SizeEq!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        unsafe impl < T > SizeEq < T > for MaybeUninit < T > { # [inline (always)] fn cast_from_raw (t : PtrInner < '_ , T >) -> PtrInner < '_ , MaybeUninit < T > > { unsafe { cast ! (t) } } }
    };
}

impl_391!();