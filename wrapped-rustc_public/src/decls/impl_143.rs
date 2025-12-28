macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: UserTypeProjection { type T = crate :: mir :: UserTypeProjection ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { UserTypeProjection { base : self . base . as_usize () , projection : opaque (& self . projs) } } }
    };
}

impl_143!();