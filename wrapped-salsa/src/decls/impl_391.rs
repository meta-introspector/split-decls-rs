macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_391 {
    () => {
        deps!();
        unsafe impl < K , V , S > Update for hashbrown :: HashMap < K , V , S > where K : Update + Eq + Hash , V : Update , S : BuildHasher , { unsafe fn maybe_update (old_pointer : * mut Self , new_map : Self) -> bool { maybe_update_map ! (old_pointer , new_map) } }
    };
}

impl_391!()