macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        unsafe impl < T , const N : usize > Update for [T ; N] where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_vec : Self) -> bool { let old_pointer : * mut T = unsafe { std :: ptr :: addr_of_mut ! ((* old_pointer) [0]) } ; let mut changed = false ; for (new_element , i) in new_vec . into_iter () . zip (0 ..) { changed |= unsafe { T :: maybe_update (old_pointer . add (i) , new_element) } ; } changed } }
    };
}

impl_399!();