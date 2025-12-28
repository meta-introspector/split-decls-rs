macro_rules! deps {
    () => {
        SockAddr!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl PartialEq for SockAddr { fn eq (& self , other : & Self) -> bool { self . as_bytes () == other . as_bytes () } }
    };
}

impl_14!()