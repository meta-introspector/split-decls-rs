macro_rules! deps {
    () => {
        FromZeros!();
    };
}

macro_rules! alloc_support {
    () => {
        deps!();
        # [cfg (feature = "alloc")] # [cfg_attr (doc_cfg , doc (cfg (feature = "alloc")))] # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] mod alloc_support { use super :: * ; # [doc = " Extends a `Vec<T>` by pushing `additional` new items onto the end of the"] # [doc = " vector. The new items are initialized with zeros."] # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] # [doc (hidden)] # [deprecated (since = "0.8.0" , note = "moved to `FromZeros`")] # [inline (always)] pub fn extend_vec_zeroed < T : FromZeros > (v : & mut Vec < T > , additional : usize ,) -> Result < () , AllocError > { < T as FromZeros > :: extend_vec_zeroed (v , additional) } # [doc = " Inserts `additional` new items into `Vec<T>` at `position`. The new"] # [doc = " items are initialized with zeros."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if `position > v.len()`."] # [cfg (zerocopy_panic_in_const_and_vec_try_reserve_1_57_0)] # [doc (hidden)] # [deprecated (since = "0.8.0" , note = "moved to `FromZeros`")] # [inline (always)] pub fn insert_vec_zeroed < T : FromZeros > (v : & mut Vec < T > , position : usize , additional : usize ,) -> Result < () , AllocError > { < T as FromZeros > :: insert_vec_zeroed (v , position , additional) } }
    };
}

alloc_support!()