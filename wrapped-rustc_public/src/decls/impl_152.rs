macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: InlineAsmOperand < 'tcx > { type T = crate :: mir :: InlineAsmOperand ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: InlineAsmOperand ; let (in_value , out_place) = match self { InlineAsmOperand :: In { value , .. } => (Some (value . stable (tables , cx)) , None) , InlineAsmOperand :: Out { place , .. } => { (None , place . map (| place | place . stable (tables , cx))) } InlineAsmOperand :: InOut { in_value , out_place , .. } => { (Some (in_value . stable (tables , cx)) , out_place . map (| place | place . stable (tables , cx))) } InlineAsmOperand :: Const { .. } | InlineAsmOperand :: SymFn { .. } | InlineAsmOperand :: SymStatic { .. } | InlineAsmOperand :: Label { .. } => (None , None) , } ; crate :: mir :: InlineAsmOperand { in_value , out_place , raw_rpr : format ! ("{self:?}") } } }
    };
}

impl_152!();