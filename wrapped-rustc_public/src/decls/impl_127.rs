macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: VarDebugInfoFragment < 'tcx > { type T = crate :: mir :: VarDebugInfoFragment ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { VarDebugInfoFragment { ty : self . ty . stable (tables , cx) , projection : self . projection . iter () . map (| e | e . stable (tables , cx)) . collect () , } } }
    };
}

impl_127!()