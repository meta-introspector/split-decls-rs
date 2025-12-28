macro_rules! deps {
    () => {
        GenericArgs!();
        Ty!();
        ParamTy!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl std :: ops :: Index < ParamTy > for GenericArgs { type Output = Ty ; fn index (& self , index : ParamTy) -> & Self :: Output { self . 0 [index . index as usize] . expect_ty () } }
    };
}

impl_378!()