macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < 'tcx , T , E > Stable < 'tcx > for Result < T , E > where T : Stable < 'tcx > , E : Stable < 'tcx > , { type T = Result < T :: T , E :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { Ok (val) => Ok (val . stable (tables , cx)) , Err (error) => Err (error . stable (tables , cx)) , } } }
    };
}

impl_232!();