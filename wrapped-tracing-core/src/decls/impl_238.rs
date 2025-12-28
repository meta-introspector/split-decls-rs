macro_rules! deps {
    () => {
        Current!();
        Id!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl < 'a > From < & 'a Current > for Option < & 'a Id > { fn from (cur : & 'a Current) -> Self { cur . id () } }
    };
}

impl_238!();