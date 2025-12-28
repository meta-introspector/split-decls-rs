macro_rules! deps {
    () => {
        Stable!();
        Ty!();
        BridgeTys!();
        TermKind!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: TermKind < 'tcx > { type T = crate :: ty :: TermKind ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { use crate :: ty :: TermKind ; match self { ty :: TermKind :: Ty (ty) => TermKind :: Type (ty . stable (tables , cx)) , ty :: TermKind :: Const (cnst) => { let cnst = cnst . stable (tables , cx) ; TermKind :: Const (cnst) } } } }
    };
}

impl_169!();