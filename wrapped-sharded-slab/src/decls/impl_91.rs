macro_rules! deps {
    () => {
        Config!();
        Generation!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl < C : cfg :: Config > PartialEq for Generation < C > { fn eq (& self , other : & Self) -> bool { self . value == other . value } }
    };
}

impl_91!()