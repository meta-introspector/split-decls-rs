macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl PartialEq < Name > for String { fn eq (& self , other : & Name) -> bool { self == other . as_str () } }
    };
}

impl_92!()