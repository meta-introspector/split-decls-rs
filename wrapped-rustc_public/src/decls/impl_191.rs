macro_rules! deps {
    () => {
        ParamTy!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_191 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ParamTy { type T = crate :: ty :: ParamTy ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: ParamTy ; ParamTy { index : self . index , name : self . name . to_string () } } }
    };
}

impl_191!()