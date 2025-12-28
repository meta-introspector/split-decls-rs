macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        Align!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Align { type T = Align ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . bytes () } }
    };
}

impl_112!()