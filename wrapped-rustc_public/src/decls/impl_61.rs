macro_rules! deps {
    () => {
        RustcInternal!();
        BridgeTys!();
        TyKind!();
        InternalCx!();
        RigidTy!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl RustcInternal for RigidTy { type T < 'tcx > = rustc_ty :: TyKind < 'tcx > ; fn internal < 'tcx > (& self , tables : & mut Tables < '_ , BridgeTys > , tcx : impl InternalCx < 'tcx > ,) -> Self :: T < 'tcx > { match self { RigidTy :: Bool => rustc_ty :: TyKind :: Bool , RigidTy :: Char => rustc_ty :: TyKind :: Char , RigidTy :: Int (int_ty) => rustc_ty :: TyKind :: Int (int_ty . internal (tables , tcx)) , RigidTy :: Uint (uint_ty) => rustc_ty :: TyKind :: Uint (uint_ty . internal (tables , tcx)) , RigidTy :: Float (float_ty) => rustc_ty :: TyKind :: Float (float_ty . internal (tables , tcx)) , RigidTy :: Never => rustc_ty :: TyKind :: Never , RigidTy :: Array (ty , cnst) => { rustc_ty :: TyKind :: Array (ty . internal (tables , tcx) , cnst . internal (tables , tcx)) } RigidTy :: Pat (ty , pat) => { rustc_ty :: TyKind :: Pat (ty . internal (tables , tcx) , pat . internal (tables , tcx)) } RigidTy :: Adt (def , args) => { rustc_ty :: TyKind :: Adt (def . internal (tables , tcx) , args . internal (tables , tcx)) } RigidTy :: Str => rustc_ty :: TyKind :: Str , RigidTy :: Slice (ty) => rustc_ty :: TyKind :: Slice (ty . internal (tables , tcx)) , RigidTy :: RawPtr (ty , mutability) => { rustc_ty :: TyKind :: RawPtr (ty . internal (tables , tcx) , mutability . internal (tables , tcx)) } RigidTy :: Ref (region , ty , mutability) => rustc_ty :: TyKind :: Ref (region . internal (tables , tcx) , ty . internal (tables , tcx) , mutability . internal (tables , tcx) ,) , RigidTy :: Foreign (def) => rustc_ty :: TyKind :: Foreign (def . 0 . internal (tables , tcx)) , RigidTy :: FnDef (def , args) => { rustc_ty :: TyKind :: FnDef (def . 0 . internal (tables , tcx) , args . internal (tables , tcx)) } RigidTy :: FnPtr (sig) => { let (sig_tys , hdr) = sig . internal (tables , tcx) . split () ; rustc_ty :: TyKind :: FnPtr (sig_tys , hdr) } RigidTy :: Closure (def , args) => { rustc_ty :: TyKind :: Closure (def . 0 . internal (tables , tcx) , args . internal (tables , tcx)) } RigidTy :: Coroutine (def , args) => { rustc_ty :: TyKind :: Coroutine (def . 0 . internal (tables , tcx) , args . internal (tables , tcx)) } RigidTy :: CoroutineClosure (def , args) => rustc_ty :: TyKind :: CoroutineClosure (def . 0 . internal (tables , tcx) , args . internal (tables , tcx) ,) , RigidTy :: CoroutineWitness (def , args) => rustc_ty :: TyKind :: CoroutineWitness (def . 0 . internal (tables , tcx) , args . internal (tables , tcx) ,) , RigidTy :: Dynamic (predicate , region , dyn_kind) => rustc_ty :: TyKind :: Dynamic (tcx . mk_poly_existential_predicates (& predicate . internal (tables , tcx)) , region . internal (tables , tcx) , dyn_kind . internal (tables , tcx) ,) , RigidTy :: Tuple (tys) => { rustc_ty :: TyKind :: Tuple (tcx . mk_type_list (& tys . internal (tables , tcx))) } } } }
    };
}

impl_61!();