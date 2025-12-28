macro_rules! deps {
    () => {
        GenericParamDefKind!();
        BridgeTys!();
        Stable!();
    };
}

macro_rules! impl_197 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_middle :: ty :: GenericParamDefKind { type T = crate :: ty :: GenericParamDefKind ; fn stable (& self , _ : & mut Tables < '_ , BridgeTys > , _ : & CompilerCtxt < '_ , BridgeTys >) -> Self :: T { use crate :: ty :: GenericParamDefKind ; match * self { ty :: GenericParamDefKind :: Lifetime => GenericParamDefKind :: Lifetime , ty :: GenericParamDefKind :: Type { has_default , synthetic } => { GenericParamDefKind :: Type { has_default , synthetic } } ty :: GenericParamDefKind :: Const { has_default } => { GenericParamDefKind :: Const { has_default } } } } }
    };
}

impl_197!();