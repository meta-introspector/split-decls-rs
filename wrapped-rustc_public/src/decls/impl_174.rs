macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        FieldDef!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: FieldDef { type T = crate :: ty :: FieldDef ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: ty :: FieldDef { def : tables . create_def_id (self . did) , name : self . name . stable (tables , cx) , } } }
    };
}

impl_174!()