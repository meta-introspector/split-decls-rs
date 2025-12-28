macro_rules! deps {
    () => {
        Stable!();
        WrappingRange!();
        BridgeTys!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: WrappingRange { type T = WrappingRange ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { WrappingRange { start : self . start , end : self . end } } }
    };
}

impl_118!()