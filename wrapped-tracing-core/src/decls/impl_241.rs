macro_rules! deps {
    () => {
        Metadata!();
        Current!();
    };
}

macro_rules! impl_241 {
    () => {
        deps!();
        impl < 'a > From < & 'a Current > for Option < & 'static Metadata < 'static > > { fn from (cur : & 'a Current) -> Self { cur . metadata () } }
    };
}

impl_241!();