macro_rules! deps {
    () => {
        Name!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl Ord for Name { fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& self . value , & other . value) } }
    };
}

impl_67!();