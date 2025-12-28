macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_224 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for FieldIdx { type T = usize ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . as_usize () } }
    };
}

impl_224!()