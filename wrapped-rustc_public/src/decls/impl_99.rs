macro_rules! deps {
    () => {
        Endian!();
        Stable!();
        BridgeTys!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Endian { type T = crate :: target :: Endian ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { match self { rustc_abi :: Endian :: Little => crate :: target :: Endian :: Little , rustc_abi :: Endian :: Big => crate :: target :: Endian :: Big , } } }
    };
}

impl_99!();