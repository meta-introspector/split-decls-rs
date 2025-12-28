macro_rules! deps {
    () => {
        BridgeTys!();
        ReprFlags!();
        Stable!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: ReprFlags { type T = ReprFlags ; fn stable < 'cx > (& self , _tables : & mut Tables < 'cx , BridgeTys > , _cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { ReprFlags { is_simd : self . intersects (Self :: IS_SIMD) , is_c : self . intersects (Self :: IS_C) , is_transparent : self . intersects (Self :: IS_TRANSPARENT) , is_linear : self . intersects (Self :: IS_LINEAR) , } } }
    };
}

impl_119!()