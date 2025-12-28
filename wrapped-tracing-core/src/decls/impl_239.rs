macro_rules! deps {
    () => {
        Id!();
        Current!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl < 'a > From < & 'a Current > for Option < Id > { fn from (cur : & 'a Current) -> Self { cur . id () . cloned () } }
    };
}

impl_239!();