macro_rules! deps {
    () => {
        Stable!();
        ForeignModule!();
        BridgeTys!();
    };
}

macro_rules! impl_216 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_session :: cstore :: ForeignModule { type T = crate :: ty :: ForeignModule ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: ForeignModule { def_id : tables . foreign_module_def (self . def_id) , abi : self . abi . stable (tables , cx) , } } }
    };
}

impl_216!()