macro_rules! deps {
    () => {
        IsFirstInputType!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl From < bool > for IsFirstInputType { fn from (b : bool) -> IsFirstInputType { match b { false => IsFirstInputType :: No , true => IsFirstInputType :: Yes , } } }
    };
}

impl_14!()