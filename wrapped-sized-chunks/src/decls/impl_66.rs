macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < const N : usize > std :: io :: Read for Chunk < u8 , N > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let read_size = buf . len () . min (self . len ()) ; if read_size == 0 { Ok (0) } else { for p in buf . iter_mut () . take (read_size) { * p = self . pop_front () ; } Ok (read_size) } } }
    };
}

impl_66!()