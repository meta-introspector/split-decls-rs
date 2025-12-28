macro_rules! adler32_fold_copy {
    () => {
        pub fn adler32_fold_copy (start_checksum : u32 , dst : & mut [u8] , src : & [u8]) -> u32 { debug_assert ! (dst . len () >= src . len () , "{} < {}" , dst . len () , src . len ()) ; dst [.. src . len ()] . copy_from_slice (src) ; adler32 (start_checksum , src) }
    };
}

adler32_fold_copy!()