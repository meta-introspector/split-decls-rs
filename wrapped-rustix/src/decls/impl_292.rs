macro_rules! impl_292 {
    () => {
        impl Errno { # [doc = " Shorthand for `std::io::Error::from(self).kind()`."] # [cfg (feature = "std")] # [inline] pub fn kind (self) -> std :: io :: ErrorKind { std :: io :: Error :: from (self) . kind () } }
    };
}

impl_292!();