macro_rules! deps {
    () => {
        Read!();
        Error!();
    };
}

macro_rules! impl_427 {
    () => {
        deps!();
        impl Read for & [u8] { fn read (& mut self , buf : & mut [u8]) -> Result < usize , Error > { let size = core :: cmp :: min (self . len () , buf . len ()) ; let (to_copy , rest) = self . split_at (size) ; if size == 1 { buf [0] = to_copy [0] ; } else { buf [.. size] . copy_from_slice (to_copy) ; } * self = rest ; Ok (size) } }
    };
}

impl_427!();