macro_rules! deps {
    () => {
        UnsupportedPlatformError!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl fmt :: Display for UnsupportedPlatformError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "unsupported platform, please file a bug at `https://github.com/Stebalien/xattr'") } }
    };
}

impl_2!()