macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_384 {
    () => {
        deps!();
        unsafe impl < T > Update for thin_vec :: ThinVec < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { maybe_update_vec ! (old_pointer , new_vec , T) } }
    };
}

impl_384!();