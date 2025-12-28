macro_rules! SeekableTake {
    () => {
        struct SeekableTake < 'a , R > { inner : & 'a mut R , inner_starting_offset : u64 , length : u64 , current_offset : u64 , }
    };
}

SeekableTake!()