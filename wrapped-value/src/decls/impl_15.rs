macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl PartialEq < Name > for String { fn eq (& self , other : & Name) -> bool { self == other . as_str () } }
    };
}

impl_15!()