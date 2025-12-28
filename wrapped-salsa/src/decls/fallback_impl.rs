macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! fallback_impl {
    () => {
        deps!();
        macro_rules ! fallback_impl { ($ ($ t : ty ,) *) => { $ (unsafe impl Update for $ t { unsafe fn maybe_update (old_pointer : * mut Self , new_value : Self) -> bool { unsafe { update_fallback (old_pointer , new_value) } } }) * } }
    };
}

fallback_impl!()