macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Terminator < 'tcx > { type T = crate :: mir :: Terminator ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: mir :: Terminator ; Terminator { kind : self . kind . stable (tables , cx) , span : self . source_info . span . stable (tables , cx) , } } }
    };
}

impl_153!();