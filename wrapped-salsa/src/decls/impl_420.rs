macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_420 {
    () => {
        deps!();
        unsafe impl < T > Update for PhantomData < T > { unsafe fn maybe_update (_old_pointer : * mut Self , _new_value : Self) -> bool { false } }
    };
}

impl_420!()