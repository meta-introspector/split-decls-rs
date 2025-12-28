macro_rules! deps {
    () => {
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_139 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for mir :: Operand < 'tcx > { type T = crate :: mir :: Operand ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use rustc_middle :: mir :: Operand :: * ; match self { Copy (place) => crate :: mir :: Operand :: Copy (place . stable (tables , cx)) , Move (place) => crate :: mir :: Operand :: Move (place . stable (tables , cx)) , Constant (c) => crate :: mir :: Operand :: Constant (c . stable (tables , cx)) , } } }
    };
}

impl_139!();