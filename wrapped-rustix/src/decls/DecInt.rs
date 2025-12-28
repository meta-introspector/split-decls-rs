macro_rules! DecInt {
    () => {
        # [doc = " Format an integer into a decimal `Path` component, without constructing a"] # [doc = " temporary `PathBuf` or `String`."] # [doc = ""] # [doc = " This is used for opening paths such as `/proc/self/fd/<fd>` on Linux."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # #[cfg(any(feature = \"fs\", feature = \"net\"))]"] # [doc = " use rustix::path::DecInt;"] # [doc = ""] # [doc = " # #[cfg(any(feature = \"fs\", feature = \"net\"))]"] # [doc = " assert_eq!("] # [doc = "     format!(\"hello {}\", DecInt::new(9876).as_ref().display()),"] # [doc = "     \"hello 9876\""] # [doc = " );"] # [doc = " ```"] # [derive (Clone)] pub struct DecInt { buf : [MaybeUninit < u8 > ; BUF_LEN] , len : NonZeroU8 , }
    };
}

DecInt!();