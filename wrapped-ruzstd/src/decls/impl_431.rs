macro_rules! deps {
    () => {
        Read!();
        Take!();
        Error!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        impl < R : Read > Read for Take < R > { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > { if self . limit == 0 { return Ok (0) ; } let at_most = (self . limit as usize) . min (buf . len ()) ; let bytes = self . inner . read (& mut buf [.. at_most]) ? ; self . limit -= bytes as u64 ; Ok (bytes) } }
    };
}

impl_431!()