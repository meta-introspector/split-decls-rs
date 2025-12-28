macro_rules! deps {
    () => {
        SeekableTake!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < 'a , R : Seek > SeekableTake < 'a , R > { pub fn new (inner : & 'a mut R , length : u64) -> io :: Result < Self > { let inner_starting_offset = inner . stream_position () ? ; Ok (Self { inner , inner_starting_offset , length , current_offset : 0 , }) } }
    };
}

impl_91!();