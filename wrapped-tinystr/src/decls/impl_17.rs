macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < const N : usize > PartialEq < TinyAsciiStr < N > > for alloc :: string :: String { fn eq (& self , other : & TinyAsciiStr < N >) -> bool { self . deref () == other . deref () } }
    };
}

impl_17!();