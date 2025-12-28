macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        ParamConst!();
    };
}

macro_rules! impl_190 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ParamConst { type T = crate :: ty :: ParamConst ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: ParamConst ; ParamConst { index : self . index , name : self . name . to_string () } } }
    };
}

impl_190!()