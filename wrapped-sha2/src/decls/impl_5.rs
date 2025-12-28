macro_rules! deps {
    () => {
        Sha256VarCore!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl UpdateCore for Sha256VarCore { # [inline] fn update_blocks (& mut self , blocks : & [Block < Self >]) { self . block_len += blocks . len () as u64 ; let blocks = Array :: cast_slice_to_core (blocks) ; compress256 (& mut self . state , blocks) ; } }
    };
}

impl_5!()