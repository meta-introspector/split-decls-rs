macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < const N : usize > PartialEq < alloc :: string :: String > for TinyAsciiStr < N > { fn eq (& self , other : & alloc :: string :: String) -> bool { self . deref () == other . deref () } }
    };
}

impl_16!();