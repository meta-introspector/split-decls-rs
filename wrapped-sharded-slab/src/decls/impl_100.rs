macro_rules! deps {
    () => {
        Lifecycle!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < C > PartialEq for Lifecycle < C > { fn eq (& self , other : & Self) -> bool { self . state == other . state } }
    };
}

impl_100!();