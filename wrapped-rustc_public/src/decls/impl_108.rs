macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        VariantsShape!();
        VariantIdx!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Variants < rustc_abi :: FieldIdx , rustc_abi :: VariantIdx > { type T = VariantsShape ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: Variants :: Single { index } => { VariantsShape :: Single { index : index . stable (tables , cx) } } rustc_abi :: Variants :: Empty => VariantsShape :: Empty , rustc_abi :: Variants :: Multiple { tag , tag_encoding , tag_field , variants } => { VariantsShape :: Multiple { tag : tag . stable (tables , cx) , tag_encoding : tag_encoding . stable (tables , cx) , tag_field : tag_field . stable (tables , cx) , variants : variants . iter () . as_slice () . stable (tables , cx) , } } } } }
    };
}

impl_108!();