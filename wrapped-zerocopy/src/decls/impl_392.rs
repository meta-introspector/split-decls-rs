macro_rules! deps {
    () => {
        SizeEq!();
        MaybeUninit!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        unsafe impl < T > SizeEq < MaybeUninit < T > > for T { # [inline (always)] fn cast_from_raw (t : PtrInner < '_ , MaybeUninit < T > >) -> PtrInner < '_ , T > { unsafe { cast ! (t) } } }
    };
}

impl_392!()