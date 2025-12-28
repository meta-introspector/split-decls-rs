macro_rules! deps {
    () => {
        Region!();
        Stable!();
        BoundVariableKind!();
        Ty!();
        BridgeTys!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: BoundVariableKind { type T = crate :: ty :: BoundVariableKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundVariableKind ; match self { ty :: BoundVariableKind :: Ty (bound_ty_kind) => { BoundVariableKind :: Ty (bound_ty_kind . stable (tables , cx)) } ty :: BoundVariableKind :: Region (bound_region_kind) => { BoundVariableKind :: Region (bound_region_kind . stable (tables , cx)) } ty :: BoundVariableKind :: Const => BoundVariableKind :: Const , } } }
    };
}

impl_182!()