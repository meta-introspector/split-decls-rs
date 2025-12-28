macro_rules! deps {
    () => {
        Running!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < F > Running < F > where F : Fn (& Metadata < '_ >) -> bool , { fn lookup_current (& self) -> Option < span :: Id > { let stack = self . current . lock () . unwrap () ; stack . last () . cloned () } }
    };
}

impl_76!();