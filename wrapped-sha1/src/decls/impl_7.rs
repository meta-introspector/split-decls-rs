macro_rules! deps {
    () => {
        Sha1Core!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl UpdateCore for Sha1Core { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u64 ; let blocks = Array :: cast_slice_to_core (blocks) ; compress (& mut self . h , blocks) ; } }
    };
}

impl_7!()