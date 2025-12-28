macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < 'a > From < & 'a std :: path :: PathBuf > for Bin { fn from (other : & 'a std :: path :: PathBuf) -> Self { Self :: Path (other . clone ()) } }
    };
}

impl_31!();