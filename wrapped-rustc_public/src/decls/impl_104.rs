macro_rules! deps {
    () => {
        Stable!();
        ArgAbi!();
        Ty!();
        BridgeTys!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for callconv :: ArgAbi < 'tcx , ty :: Ty < 'tcx > > { type T = ArgAbi ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { ArgAbi { ty : self . layout . ty . stable (tables , cx) , layout : self . layout . layout . stable (tables , cx) , mode : self . mode . stable (tables , cx) , } } }
    };
}

impl_104!();