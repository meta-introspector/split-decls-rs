macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_401 {
    () => {
        deps!();
        impl PartialEq for Lifetime { fn eq (& self , other : & Lifetime) -> bool { self . ident . eq (& other . ident) } }
    };
}

impl_401!();