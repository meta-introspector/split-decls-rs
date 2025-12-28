macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_385 {
    () => {
        deps!();
        unsafe impl < T , const N : usize > Update for smallvec :: SmallVec < T , N > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { maybe_update_vec ! (old_pointer , new_vec , T) } }
    };
}

impl_385!();