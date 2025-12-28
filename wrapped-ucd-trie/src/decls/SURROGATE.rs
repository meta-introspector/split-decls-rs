macro_rules! SURROGATE {
    () => {
        pub const SURROGATE : & 'static [(u32 , u32)] = & [(55296 , 57343)] ;
    };
}

SURROGATE!()