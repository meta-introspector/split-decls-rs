macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_383 {
    () => {
        deps!();
        unsafe impl < T > Update for Vec < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { maybe_update_vec ! (old_pointer , new_vec , T) } }
    };
}

impl_383!();