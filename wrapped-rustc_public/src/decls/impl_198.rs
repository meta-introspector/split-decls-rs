macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        GenericParamDef!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_middle :: ty :: GenericParamDef { type T = crate :: ty :: GenericParamDef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { GenericParamDef { name : self . name . to_string () , def_id : tables . generic_def (self . def_id) , index : self . index , pure_wrt_drop : self . pure_wrt_drop , kind : self . kind . stable (tables , cx) , } } }
    };
}

impl_198!();