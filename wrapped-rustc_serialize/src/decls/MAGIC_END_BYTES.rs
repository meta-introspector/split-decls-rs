macro_rules! MAGIC_END_BYTES {
    () => {
        pub const MAGIC_END_BYTES : & [u8] = b"rust-end-file" ;
    };
}

MAGIC_END_BYTES!();