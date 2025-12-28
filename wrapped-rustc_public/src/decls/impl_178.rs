macro_rules! deps {
    () => {
        EarlyBinder!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'tcx , S , V > Stable < 'tcx > for ty :: EarlyBinder < 'tcx , S > where S : Stable < 'tcx , T = V > , { type T = crate :: ty :: EarlyBinder < V > ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: EarlyBinder ; EarlyBinder { value : self . as_ref () . skip_binder () . stable (tables , cx) } } }
    };
}

impl_178!();