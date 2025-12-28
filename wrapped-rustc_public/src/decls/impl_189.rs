macro_rules! deps {
    () => {
        TyConstKind!();
        Placeholder!();
        BridgeTys!();
        TyConst!();
        Error!();
        Stable!();
    };
}

macro_rules! impl_189 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for ty :: Const < 'tcx > { type T = crate :: ty :: TyConst ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let ct = cx . lift (* self) . unwrap () ; let kind = match ct . kind () { ty :: ConstKind :: Value (cv) => { let const_val = cx . valtree_to_const_val (cv) ; if matches ! (const_val , mir :: ConstValue :: ZeroSized) { crate :: ty :: TyConstKind :: ZSTValue (cv . ty . stable (tables , cx)) } else { crate :: ty :: TyConstKind :: Value (cv . ty . stable (tables , cx) , alloc :: new_allocation (cv . ty , const_val , tables , cx) ,) } } ty :: ConstKind :: Param (param) => crate :: ty :: TyConstKind :: Param (param . stable (tables , cx)) , ty :: ConstKind :: Unevaluated (uv) => crate :: ty :: TyConstKind :: Unevaluated (tables . const_def (uv . def) , uv . args . stable (tables , cx) ,) , ty :: ConstKind :: Error (_) => unreachable ! () , ty :: ConstKind :: Infer (_) => unreachable ! () , ty :: ConstKind :: Bound (_ , _) => unimplemented ! () , ty :: ConstKind :: Placeholder (_) => unimplemented ! () , ty :: ConstKind :: Expr (_) => unimplemented ! () , } ; let id = tables . intern_ty_const (ct) ; crate :: ty :: TyConst :: new (kind , id) } }
    };
}

impl_189!();