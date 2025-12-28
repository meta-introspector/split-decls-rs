macro_rules! Location {
    () => {
        pub struct Location { pub file : & 'static str , pub line : u32 , }
    };
}

Location!()