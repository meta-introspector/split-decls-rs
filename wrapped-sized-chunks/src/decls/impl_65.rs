macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < const N : usize > io :: Write for Chunk < u8 , N > { fn write (& mut self , buf : & [u8]) -> io :: Result < usize > { let old_len = self . len () ; self . extend (buf . iter () . cloned () . take (N - old_len)) ; Ok (self . len () - old_len) } fn flush (& mut self) -> io :: Result < () > { Ok (()) } }
    };
}

impl_65!()