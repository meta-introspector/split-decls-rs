macro_rules! deps {
    () => {
        Tid!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < C > PartialEq for Tid < C > { fn eq (& self , other : & Self) -> bool { self . id == other . id } }
    };
}

impl_170!()