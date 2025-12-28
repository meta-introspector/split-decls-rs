macro_rules! deps {
    () => {
        SeekableTake!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < R : Seek > Seek for SeekableTake < '_ , R > { fn seek (& mut self , pos : SeekFrom) -> io :: Result < u64 > { let offset = match pos { SeekFrom :: Start (offset) => Some (offset) , SeekFrom :: End (offset) => self . length . checked_add_signed (offset) , SeekFrom :: Current (offset) => self . current_offset . checked_add_signed (offset) , } ; match offset { None => Err (io :: Error :: new (io :: ErrorKind :: InvalidInput , "invalid seek to a negative or overflowing position" ,)) , Some (offset) => { let clamped_offset = std :: cmp :: min (self . length , offset) ; let new_inner_offset = self . inner . seek (SeekFrom :: Start (self . inner_starting_offset + clamped_offset)) ? ; self . current_offset = new_inner_offset - self . inner_starting_offset ; Ok (self . current_offset) } } } }
    };
}

impl_92!();