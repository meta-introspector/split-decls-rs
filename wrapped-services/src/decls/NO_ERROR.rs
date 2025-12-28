macro_rules! deps {
    () => {
        WIN32_ERROR!();
    };
}

macro_rules! NO_ERROR {
    () => {
        deps!();
        pub const NO_ERROR : WIN32_ERROR = 0u32 ;
    };
}

NO_ERROR!()