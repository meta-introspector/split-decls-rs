macro_rules! deps {
    () => {
        Stable!();
        BridgeTys!();
        GenericArgKind!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: GenericArgKind < 'tcx > { type T = crate :: ty :: GenericArgKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: GenericArgKind ; match self { ty :: GenericArgKind :: Lifetime (region) => { GenericArgKind :: Lifetime (region . stable (tables , cx)) } ty :: GenericArgKind :: Type (ty) => GenericArgKind :: Type (ty . stable (tables , cx)) , ty :: GenericArgKind :: Const (cnst) => GenericArgKind :: Const (cnst . stable (tables , cx)) , } } }
    };
}

impl_176!()