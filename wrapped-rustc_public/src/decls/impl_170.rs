macro_rules! deps {
    () => {
        BridgeTys!();
        ExistentialProjection!();
        Stable!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: ExistentialProjection < 'tcx > { type T = crate :: ty :: ExistentialProjection ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: ExistentialProjection { def_id , args , term , .. } = self ; crate :: ty :: ExistentialProjection { def_id : tables . trait_def (* def_id) , generic_args : args . stable (tables , cx) , term : term . kind () . stable (tables , cx) , } } }
    };
}

impl_170!();