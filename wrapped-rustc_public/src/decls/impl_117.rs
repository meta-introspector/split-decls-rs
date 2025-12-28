macro_rules! deps {
    () => {
        Stable!();
        FloatLength!();
        BridgeTys!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Float { type T = FloatLength ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_abi :: Float :: F16 => FloatLength :: F16 , rustc_abi :: Float :: F32 => FloatLength :: F32 , rustc_abi :: Float :: F64 => FloatLength :: F64 , rustc_abi :: Float :: F128 => FloatLength :: F128 , } } }
    };
}

impl_117!()