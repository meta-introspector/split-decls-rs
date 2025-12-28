macro_rules! deps {
    () => {
        BridgeTys!();
        BoundTy!();
        Stable!();
    };
}

macro_rules! impl_192 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: BoundTy { type T = crate :: ty :: BoundTy ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundTy ; BoundTy { var : self . var . as_usize () , kind : self . kind . stable (tables , cx) } } }
    };
}

impl_192!();