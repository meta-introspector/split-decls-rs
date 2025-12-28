macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: UserTypeAnnotationIndex { type T = usize ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { self . as_usize () } }
    };
}

impl_172!()