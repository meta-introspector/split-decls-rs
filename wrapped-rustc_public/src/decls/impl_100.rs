macro_rules! deps {
    () => {
        TyAndLayout!();
        BridgeTys!();
        Ty!();
        Stable!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: TyAndLayout < 'tcx , ty :: Ty < 'tcx > > { type T = TyAndLayout ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { TyAndLayout { ty : self . ty . stable (tables , cx) , layout : self . layout . stable (tables , cx) } } }
    };
}

impl_100!()