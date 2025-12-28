macro_rules! deps {
    () => {
        BridgeTys!();
        AssocKind!();
        AssocTypeData!();
        Stable!();
    };
}

macro_rules! impl_217 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: AssocKind { type T = crate :: ty :: AssocKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: { AssocKind , AssocTypeData } ; match * self { ty :: AssocKind :: Const { name } => AssocKind :: Const { name : name . to_string () } , ty :: AssocKind :: Fn { name , has_self } => { AssocKind :: Fn { name : name . to_string () , has_self } } ty :: AssocKind :: Type { data } => AssocKind :: Type { data : match data { ty :: AssocTypeData :: Normal (name) => AssocTypeData :: Normal (name . to_string ()) , ty :: AssocTypeData :: Rpitit (rpitit) => { AssocTypeData :: Rpitit (rpitit . stable (tables , cx)) } } , } , } } }
    };
}

impl_217!();