macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        # [cfg (feature = "rayon")] unsafe impl < L , R > Update for Either < L , R > where L : Update , R : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_value : Self) -> bool { let old_value = unsafe { & mut * old_pointer } ; match (old_value , new_value) { (Either :: Left (old) , Either :: Left (new)) => unsafe { L :: maybe_update (old , new) } , (Either :: Right (old) , Either :: Right (new)) => unsafe { R :: maybe_update (old , new) } , (old_value , new_value) => { * old_value = new_value ; true } } } }
    };
}

impl_401!()