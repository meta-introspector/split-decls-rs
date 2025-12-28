macro_rules! deps {
    () => {
        AliasTy!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AliasTy < 'tcx > { type T = crate :: ty :: AliasTy ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ty :: AliasTy { args , def_id , .. } = self ; crate :: ty :: AliasTy { def_id : tables . alias_def (* def_id) , args : args . stable (tables , cx) } } }
    };
}

impl_164!();