macro_rules! deps {
    () => {
        SizeEq!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        unsafe impl < T : ? Sized > SizeEq < T > for T { # [inline (always)] fn cast_from_raw (t : PtrInner < '_ , T >) -> PtrInner < '_ , T > { t } }
    };
}

impl_376!()