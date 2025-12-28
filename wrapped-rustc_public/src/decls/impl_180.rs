macro_rules! deps {
    () => {
        Stable!();
        BoundTyKind!();
        BridgeTys!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: BoundTyKind { type T = crate :: ty :: BoundTyKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: BoundTyKind ; match self { ty :: BoundTyKind :: Anon => BoundTyKind :: Anon , ty :: BoundTyKind :: Param (def_id) => { BoundTyKind :: Param (tables . param_def (* def_id) , cx . tcx . item_name (* def_id) . to_string ()) } } } }
    };
}

impl_180!()