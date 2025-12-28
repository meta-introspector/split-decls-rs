macro_rules! deps {
    () => {
        Tys!();
        TypeSuperFoldable!();
        DynKind!();
        AliasTyKind!();
        CollectAndApply!();
        GenericArgs!();
        Term!();
        DefId!();
        Flags!();
        Const!();
        GenericArg!();
        AliasTy!();
        Binder!();
        BoundExistentialPredicates!();
        TyKind!();
        FnSig!();
        ClosureKind!();
        IntoKind!();
        Interner!();
        InferTy!();
        Region!();
        TypeSuperVisitable!();
        AdtDef!();
        Relate!();
    };
}

macro_rules! Ty {
    () => {
        deps!();
        pub trait Ty < I : Interner < Ty = Self > > : Copy + Debug + Hash + Eq + Into < I :: GenericArg > + Into < I :: Term > + IntoKind < Kind = ty :: TyKind < I > > + TypeSuperVisitable < I > + TypeSuperFoldable < I > + Relate < I > + Flags { fn new_unit (interner : I) -> Self ; fn new_bool (interner : I) -> Self ; fn new_u8 (interner : I) -> Self ; fn new_usize (interner : I) -> Self ; fn new_infer (interner : I , var : ty :: InferTy) -> Self ; fn new_var (interner : I , var : ty :: TyVid) -> Self ; fn new_param (interner : I , param : I :: ParamTy) -> Self ; fn new_placeholder (interner : I , param : I :: PlaceholderTy) -> Self ; fn new_bound (interner : I , debruijn : ty :: DebruijnIndex , var : I :: BoundTy) -> Self ; fn new_anon_bound (interner : I , debruijn : ty :: DebruijnIndex , var : ty :: BoundVar) -> Self ; fn new_alias (interner : I , kind : ty :: AliasTyKind , alias_ty : ty :: AliasTy < I >) -> Self ; fn new_projection_from_args (interner : I , def_id : I :: DefId , args : I :: GenericArgs) -> Self { Ty :: new_alias (interner , ty :: AliasTyKind :: Projection , ty :: AliasTy :: new_from_args (interner , def_id , args) ,) } fn new_projection (interner : I , def_id : I :: DefId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> Self { Ty :: new_alias (interner , ty :: AliasTyKind :: Projection , ty :: AliasTy :: new (interner , def_id , args) ,) } fn new_error (interner : I , guar : I :: ErrorGuaranteed) -> Self ; fn new_adt (interner : I , adt_def : I :: AdtDef , args : I :: GenericArgs) -> Self ; fn new_foreign (interner : I , def_id : I :: ForeignId) -> Self ; fn new_dynamic (interner : I , preds : I :: BoundExistentialPredicates , region : I :: Region , kind : ty :: DynKind ,) -> Self ; fn new_coroutine (interner : I , def_id : I :: CoroutineId , args : I :: GenericArgs) -> Self ; fn new_coroutine_closure (interner : I , def_id : I :: CoroutineClosureId , args : I :: GenericArgs ,) -> Self ; fn new_closure (interner : I , def_id : I :: ClosureId , args : I :: GenericArgs) -> Self ; fn new_coroutine_witness (interner : I , def_id : I :: CoroutineId , args : I :: GenericArgs) -> Self ; fn new_coroutine_witness_for_coroutine (interner : I , def_id : I :: CoroutineId , coroutine_args : I :: GenericArgs ,) -> Self ; fn new_ptr (interner : I , ty : Self , mutbl : Mutability) -> Self ; fn new_ref (interner : I , region : I :: Region , ty : Self , mutbl : Mutability) -> Self ; fn new_array_with_const_len (interner : I , ty : Self , len : I :: Const) -> Self ; fn new_slice (interner : I , ty : Self) -> Self ; fn new_tup (interner : I , tys : & [I :: Ty]) -> Self ; fn new_tup_from_iter < It , T > (interner : I , iter : It) -> T :: Output where It : Iterator < Item = T > , T : CollectAndApply < Self , Self > ; fn new_fn_def (interner : I , def_id : I :: FunctionId , args : I :: GenericArgs) -> Self ; fn new_fn_ptr (interner : I , sig : ty :: Binder < I , ty :: FnSig < I > >) -> Self ; fn new_pat (interner : I , ty : Self , pat : I :: Pat) -> Self ; fn new_unsafe_binder (interner : I , ty : ty :: Binder < I , I :: Ty >) -> Self ; fn tuple_fields (self) -> I :: Tys ; fn to_opt_closure_kind (self) -> Option < ty :: ClosureKind > ; fn from_closure_kind (interner : I , kind : ty :: ClosureKind) -> Self ; fn from_coroutine_closure_kind (interner : I , kind : ty :: ClosureKind) -> Self ; fn is_ty_var (self) -> bool { matches ! (self . kind () , ty :: Infer (ty :: TyVar (_))) } fn is_ty_error (self) -> bool { matches ! (self . kind () , ty :: Error (_)) } fn is_floating_point (self) -> bool { matches ! (self . kind () , ty :: Float (_) | ty :: Infer (ty :: FloatVar (_))) } fn is_integral (self) -> bool { matches ! (self . kind () , ty :: Infer (ty :: IntVar (_)) | ty :: Int (_) | ty :: Uint (_)) } fn is_fn_ptr (self) -> bool { matches ! (self . kind () , ty :: FnPtr (..)) } # [doc = " Checks whether this type is an ADT that has unsafe fields."] fn has_unsafe_fields (self) -> bool ; fn fn_sig (self , interner : I) -> ty :: Binder < I , ty :: FnSig < I > > { self . kind () . fn_sig (interner) } fn discriminant_ty (self , interner : I) -> I :: Ty ; fn is_known_rigid (self) -> bool { self . kind () . is_known_rigid () } fn is_guaranteed_unsized_raw (self) -> bool { match self . kind () { ty :: Dynamic (_ , _ , ty :: Dyn) | ty :: Slice (_) | ty :: Str => true , ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Adt (_ , _) | ty :: Foreign (_) | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (_ , _) | ty :: UnsafeBinder (_) | ty :: Closure (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (_ , _) | ty :: Never | ty :: Tuple (_) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Bound (_ , _) | ty :: Placeholder (_) | ty :: Infer (_) | ty :: Error (_) => false , } } }
    };
}

Ty!()