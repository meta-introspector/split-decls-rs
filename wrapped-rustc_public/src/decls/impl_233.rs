macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < 'tcx , T > Stable < 'tcx > for & [T] where T : Stable < 'tcx > , { type T = Vec < T :: T > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { self . iter () . map (| e | e . stable (tables , cx)) . collect () } }
    };
}

impl_233!()