macro_rules! deps {
    () => {
        ConstantKind!();
        MirConst!();
        Ty!();
        Stable!();
        BridgeTys!();
        UnevaluatedConst!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < 'tcx > Stable < 'tcx > for rustc_middle :: mir :: Const < 'tcx > { type T = crate :: ty :: MirConst ; fn stable < 'cx > (& self , tables : & mut Tables < 'cx , BridgeTys > , cx : & CompilerCtxt < 'cx , BridgeTys > ,) -> Self :: T { let id = tables . intern_mir_const (cx . lift (* self) . unwrap ()) ; match * self { mir :: Const :: Ty (ty , c) => MirConst :: new (crate :: ty :: ConstantKind :: Ty (c . stable (tables , cx)) , ty . stable (tables , cx) , id ,) , mir :: Const :: Unevaluated (unev_const , ty) => { let kind = crate :: ty :: ConstantKind :: Unevaluated (crate :: ty :: UnevaluatedConst { def : tables . const_def (unev_const . def) , args : unev_const . args . stable (tables , cx) , promoted : unev_const . promoted . map (| u | u . as_u32 ()) , }) ; let ty = ty . stable (tables , cx) ; MirConst :: new (kind , ty , id) } mir :: Const :: Val (mir :: ConstValue :: ZeroSized , ty) => { let ty = ty . stable (tables , cx) ; MirConst :: new (ConstantKind :: ZeroSized , ty , id) } mir :: Const :: Val (val , ty) => { let ty = cx . lift (ty) . unwrap () ; let val = cx . lift (val) . unwrap () ; let kind = ConstantKind :: Allocated (alloc :: new_allocation (ty , val , tables , cx)) ; let ty = ty . stable (tables , cx) ; MirConst :: new (kind , ty , id) } } } }
    };
}

impl_159!()