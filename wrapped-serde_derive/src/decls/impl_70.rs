macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl PartialEq for Name { fn eq (& self , other : & Self) -> bool { self . value == other . value } }
    };
}

impl_70!();