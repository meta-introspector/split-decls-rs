macro_rules! deps {
    () => {
        IntoIter!();
        ThinVec!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < T > Drop for IntoIter < T > { # [inline] fn drop (& mut self) { # [cold] # [inline (never)] fn drop_non_singleton < T > (this : & mut IntoIter < T >) { unsafe { let mut vec = mem :: replace (& mut this . vec , ThinVec :: new ()) ; ptr :: drop_in_place (& mut vec [this . start ..]) ; vec . set_len_non_singleton (0) } } if ! self . vec . is_singleton () { drop_non_singleton (self) ; } } }
    };
}

impl_67!()