macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        Scalar!();
        ValueAbi!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: BackendRepr { type T = ValueAbi ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match * self { rustc_abi :: BackendRepr :: Scalar (scalar) => ValueAbi :: Scalar (scalar . stable (tables , cx)) , rustc_abi :: BackendRepr :: ScalarPair (first , second) => { ValueAbi :: ScalarPair (first . stable (tables , cx) , second . stable (tables , cx)) } rustc_abi :: BackendRepr :: SimdVector { element , count } => { ValueAbi :: Vector { element : element . stable (tables , cx) , count } } rustc_abi :: BackendRepr :: Memory { sized } => ValueAbi :: Aggregate { sized } , } } }
    };
}

impl_110!();