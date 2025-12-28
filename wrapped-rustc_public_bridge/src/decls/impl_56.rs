macro_rules! deps {
    () => {
        Bridge!();
        Tables!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'tcx , B : Bridge > Index < B :: DefId > for Tables < 'tcx , B > { type Output = DefId ; # [inline (always)] fn index (& self , index : B :: DefId) -> & Self :: Output { & self . def_ids [index] } }
    };
}

impl_56!();