macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        AliasTerm!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AliasTerm < 'tcx > { type T = crate :: ty :: AliasTerm ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: AliasTerm { args , def_id , .. } = self ; crate :: ty :: AliasTerm { def_id : tables . alias_def (* def_id) , args : args . stable (tables , cx) } } }
    };
}

impl_165!();