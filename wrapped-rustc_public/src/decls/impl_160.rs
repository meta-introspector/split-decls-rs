macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        Error!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: interpret :: ErrorHandled { type T = Error ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { bridge :: Error :: new (format ! ("{self:?}")) } }
    };
}

impl_160!()