macro_rules! deps {
    () => {
        Update!();
    };
}

macro_rules! impl_400 {
    () => {
        deps!();
        unsafe impl < T , E > Update for Result < T , E > where T : Update , E : Update , { unsafe fn maybe_update (old_pointer : * mut Self , new_value : Self) -> bool { let old_value = unsafe { & mut * old_pointer } ; match (old_value , new_value) { (Ok (old) , Ok (new)) => unsafe { T :: maybe_update (old , new) } , (Err (old) , Err (new)) => unsafe { E :: maybe_update (old , new) } , (old_value , new_value) => { * old_value = new_value ; true } } } }
    };
}

impl_400!()