macro_rules! get {
    () => {
        fn get (n : u16 , idx : u16) -> u16 { (n >> (BITS * idx)) & MASK }
    };
}

get!()