macro_rules! deps {
    () => {
        Sha512VarCore!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl UpdateCore for Sha512VarCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u128 ; let blocks = Array :: cast_slice_to_core (blocks) ; compress512 (& mut self . state , blocks) ; } }
    };
}

impl_17!();