macro_rules! deps {
    () => {
        AddressSpace!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: AddressSpace { type T = AddressSpace ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { AddressSpace (self . 0) } }
    };
}

impl_115!()