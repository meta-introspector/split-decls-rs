macro_rules! deps {
    () => {
        GenericArgs!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: GenericArgs < 'tcx > { type T = crate :: ty :: GenericArgs ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { GenericArgs (self . iter () . map (| arg | arg . kind () . stable (tables , cx)) . collect ()) } }
    };
}

impl_175!()