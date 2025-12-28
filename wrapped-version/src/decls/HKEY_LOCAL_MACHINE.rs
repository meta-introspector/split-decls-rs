macro_rules! deps {
    () => {
        HKEY!();
    };
}

macro_rules! HKEY_LOCAL_MACHINE {
    () => {
        deps!();
        pub const HKEY_LOCAL_MACHINE : HKEY = - 2147483646i32 as _ ;
    };
}

HKEY_LOCAL_MACHINE!()