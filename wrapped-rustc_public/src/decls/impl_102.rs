macro_rules! deps {
    () => {
        Stable!();
        LayoutShape!();
        VariantIdx!();
        BridgeTys!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: LayoutData < rustc_abi :: FieldIdx , rustc_abi :: VariantIdx > { type T = LayoutShape ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { LayoutShape { fields : self . fields . stable (tables , cx) , variants : self . variants . stable (tables , cx) , abi : self . backend_repr . stable (tables , cx) , abi_align : self . align . abi . stable (tables , cx) , size : self . size . stable (tables , cx) , } } }
    };
}

impl_102!()