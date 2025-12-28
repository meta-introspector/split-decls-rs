macro_rules! deps {
    () => {
        Primitive!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_abi :: Primitive { type T = Primitive ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { match self { rustc_abi :: Primitive :: Int (length , signed) => { Primitive :: Int { length : length . stable (tables , cx) , signed : * signed } } rustc_abi :: Primitive :: Float (length) => { Primitive :: Float { length : length . stable (tables , cx) } } rustc_abi :: Primitive :: Pointer (space) => Primitive :: Pointer (space . stable (tables , cx)) , } } }
    };
}

impl_114!()