macro_rules! deps {
    () => {
        FnSig!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: FnSig < 'tcx > { type T = crate :: ty :: FnSig ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: FnSig ; FnSig { inputs_and_output : self . inputs_and_output . iter () . map (| ty | ty . stable (tables , cx)) . collect () , c_variadic : self . c_variadic , safety : self . safety . stable (tables , cx) , abi : self . abi . stable (tables , cx) , } } }
    };
}

impl_179!()