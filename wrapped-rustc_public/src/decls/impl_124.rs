macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_124 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: VarDebugInfo < 'tcx > { type T = crate :: mir :: VarDebugInfo ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: VarDebugInfo { name : self . name . to_string () , source_info : self . source_info . stable (tables , cx) , composite : self . composite . as_ref () . map (| composite | composite . stable (tables , cx)) , value : self . value . stable (tables , cx) , argument_index : self . argument_index , } } }
    };
}

impl_124!();