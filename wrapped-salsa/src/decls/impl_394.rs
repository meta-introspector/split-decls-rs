macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_394 {
    () => {
        deps!();
        unsafe impl < K , S > Update for indexmap :: IndexSet < K , S > where K : Update + Eq + Hash , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_set : Self) -> bool { maybe_update_set ! (old_pointer , new_set) } }
    };
}

impl_394!();