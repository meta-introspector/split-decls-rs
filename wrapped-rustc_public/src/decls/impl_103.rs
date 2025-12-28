macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        FnAbi!();
        Ty!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for callconv :: FnAbi < 'tcx , ty :: Ty < 'tcx > > { type T = FnAbi ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { assert ! (self . args . len () >= self . fixed_count as usize) ; assert ! (! self . c_variadic || matches ! (self . conv , CanonAbi :: C)) ; FnAbi { args : self . args . as_ref () . stable (tables , cx) , ret : self . ret . stable (tables , cx) , fixed_count : self . fixed_count , conv : self . conv . stable (tables , cx) , c_variadic : self . c_variadic , } } }
    };
}

impl_103!();