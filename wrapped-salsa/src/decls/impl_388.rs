macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_388 {
    () => {
        deps!();
        unsafe impl < K > Update for BTreeSet < K > where K : Update + Eq + Ord , { unsafe fn maybe_update (old_pointer : * mut Self , new_set : Self) -> bool { maybe_update_set ! (old_pointer , new_set) } }
    };
}

impl_388!();