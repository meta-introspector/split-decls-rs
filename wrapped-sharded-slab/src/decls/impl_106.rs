macro_rules! deps {
    () => {
        RefCount!();
        Config!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < C : cfg :: Config > PartialEq for RefCount < C > { fn eq (& self , other : & Self) -> bool { self . value == other . value } }
    };
}

impl_106!();