macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < 'a > From < & 'a Id > for Option < Id > { fn from (id : & 'a Id) -> Self { Some (id . clone ()) } }
    };
}

impl_234!();