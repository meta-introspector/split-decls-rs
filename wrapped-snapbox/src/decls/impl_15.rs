macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl PartialEq for Error { fn eq (& self , other : & Self) -> bool { self . inner == other . inner } }
    };
}

impl_15!()