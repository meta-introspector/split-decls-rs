macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: ConstOperand < 'tcx > { type T = crate :: mir :: ConstOperand ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { crate :: mir :: ConstOperand { span : self . span . stable (tables , cx) , user_ty : self . user_ty . map (| u | u . as_usize ()) . or (None) , const_ : self . const_ . stable (tables , cx) , } } }
    };
}

impl_140!()