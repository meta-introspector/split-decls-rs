macro_rules! ffi {
    () => {
        pub (crate) mod ffi { pub const S_IFDIR : u32 = 0o0040000 ; pub const S_IFREG : u32 = 0o0100000 ; pub const S_IFLNK : u32 = 0o0120000 ; }
    };
}

ffi!()