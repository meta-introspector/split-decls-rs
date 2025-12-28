macro_rules! deps {
    () => {
        SliceVecDrain!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < 'p , 's , T : Default > Drop for SliceVecDrain < 'p , 's , T > { # [inline] fn drop (& mut self) { self . for_each (drop) ; let count = self . target_end - self . target_start ; let targets : & mut [T] = & mut self . parent . deref_mut () [self . target_start ..] ; targets . rotate_left (count) ; self . parent . len -= count ; } }
    };
}

impl_93!();