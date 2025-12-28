macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_320 {
    () => {
        deps!();
        impl From < Ident > for TypeParam { fn from (ident : Ident) -> Self { TypeParam { attrs : vec ! [] , ident , colon_token : None , bounds : Punctuated :: new () , eq_token : None , default : None , } } }
    };
}

impl_320!();