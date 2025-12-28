macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        unsafe impl < K , V > Update for BTreeMap < K , V > where K : Update + Eq + Ord , V : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_map : Self) -> bool { maybe_update_map ! (old_pointer , new_map) } }
    };
}

impl_395!();