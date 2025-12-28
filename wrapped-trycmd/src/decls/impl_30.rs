macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        impl From < std :: path :: PathBuf > for Bin { fn from (other : std :: path :: PathBuf) -> Self { Self :: Path (other) } }
    };
}

impl_30!();