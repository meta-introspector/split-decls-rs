macro_rules! CONTROL {
    () => {
        pub const CONTROL : & 'static [(u32 , u32)] = & [(0 , 31) , (127 , 159)] ;
    };
}

CONTROL!()