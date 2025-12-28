macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_231 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl < const N : usize > std :: io :: Read for RingBuffer < u8 , N > { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { let read_size = buf . len () . min (self . len ()) ; if read_size == 0 { Ok (0) } else { for p in buf . iter_mut () . take (read_size) { * p = self . pop_front () . unwrap () ; } Ok (read_size) } } }
    };
}

impl_231!();