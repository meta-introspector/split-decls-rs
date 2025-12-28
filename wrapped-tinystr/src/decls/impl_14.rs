macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < const N : usize > PartialEq < str > for TinyAsciiStr < N > { fn eq (& self , other : & str) -> bool { self . deref () == other } }
    };
}

impl_14!();