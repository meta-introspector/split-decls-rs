macro_rules! is_float {
    () => {
        fn is_float (raw : & str) -> bool { raw . as_bytes () . find_slice ((b'.' , b'e' , b'E')) . is_some () }
    };
}

is_float!()