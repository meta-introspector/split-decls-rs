macro_rules! deps {
    () => {
        MaybeDangling!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < T > Drop for MaybeDangling < T > { fn drop (& mut self) { unsafe { core :: ptr :: drop_in_place (self . 0 . as_mut_ptr ()) } ; } }
    };
}

impl_109!();