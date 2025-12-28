macro_rules! deps {
    () => {
        Bin!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < 'a > From < & 'a std :: path :: Path > for Bin { fn from (other : & 'a std :: path :: Path) -> Self { Self :: Path (other . to_owned ()) } }
    };
}

impl_32!()