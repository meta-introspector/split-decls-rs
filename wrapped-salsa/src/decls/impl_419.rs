macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        unsafe impl < T > Update for Option < T > where T : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_value : Self) -> bool { let old_value = unsafe { & mut * old_pointer } ; match (old_value , new_value) { (Some (old) , Some (new)) => unsafe { T :: maybe_update (old , new) } , (None , None) => false , (old_value , new_value) => { * old_value = new_value ; true } } } }
    };
}

impl_419!()