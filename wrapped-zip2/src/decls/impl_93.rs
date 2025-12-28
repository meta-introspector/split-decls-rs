macro_rules! deps {
    () => {
        SeekableTake!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < R : Read > Read for SeekableTake < '_ , R > { fn read (& mut self , buf : & mut [u8]) -> io :: Result < usize > { let written = self . inner . take (self . length - self . current_offset) . read (buf) ? ; self . current_offset += written as u64 ; Ok (written) } }
    };
}

impl_93!();