macro_rules! deps {
    () => {
        Size!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Size { type T = Size ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { Size :: from_bits (self . bits_usize ()) } }
    };
}

impl_111!()