macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
        TagEncoding!();
        VariantIdx!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: TagEncoding < rustc_abi :: VariantIdx > { type T = TagEncoding ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: TagEncoding :: Direct => TagEncoding :: Direct , rustc_abi :: TagEncoding :: Niche { untagged_variant , niche_variants , niche_start } => { TagEncoding :: Niche { untagged_variant : untagged_variant . stable (tables , cx) , niche_variants : niche_variants . stable (tables , cx) , niche_start : * niche_start , } } } } }
    };
}

impl_109!();