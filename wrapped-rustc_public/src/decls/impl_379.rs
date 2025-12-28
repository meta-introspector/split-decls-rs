macro_rules! deps {
    () => {
        TyConst!();
        GenericArgs!();
        ParamConst!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl std :: ops :: Index < ParamConst > for GenericArgs { type Output = TyConst ; fn index (& self , index : ParamConst) -> & Self :: Output { self . 0 [index . index as usize] . expect_const () } }
    };
}

impl_379!()