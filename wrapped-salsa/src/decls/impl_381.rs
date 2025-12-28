macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        unsafe impl Update for std :: convert :: Infallible { unsafe fn maybe_update (_old_pointer : * mut Self , new_value : Self) -> bool { match new_value { } } }
    };
}

impl_381!()