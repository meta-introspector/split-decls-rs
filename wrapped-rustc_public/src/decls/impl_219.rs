macro_rules! deps {
    () => {
        Stable!();
        AssocItem!();
        BridgeTys!();
    };
}

macro_rules! impl_219 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AssocItem { type T = crate :: ty :: AssocItem ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: AssocItem { def_id : tables . assoc_def (self . def_id) , kind : self . kind . stable (tables , cx) , container : self . container . stable (tables , cx) , } } }
    };
}

impl_219!()