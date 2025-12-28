macro_rules! deps {
    () => {
        IntegerType!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: IntegerType { type T = IntegerType ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: IntegerType :: Pointer (signed) => IntegerType :: Pointer { is_signed : * signed } , rustc_abi :: IntegerType :: Fixed (integer , signed) => { IntegerType :: Fixed { length : integer . stable (tables , cx) , is_signed : * signed } } } } }
    };
}

impl_120!();