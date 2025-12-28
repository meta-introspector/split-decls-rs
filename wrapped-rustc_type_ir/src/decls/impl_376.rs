macro_rules! deps {
    () => {
        Interner!();
        AliasTerm!();
        GenericArg!();
        AliasTermKind!();
        Term!();
        AliasTy!();
        GenericArgs!();
        DefId!();
        AliasTyKind!();
        Const!();
        UnevaluatedConst!();
        Ty!();
    };
}

macro_rules! impl_376 {
    () => {
        deps!();
        impl < I : Interner > AliasTerm < I > { pub fn new_from_args (interner : I , def_id : I :: DefId , args : I :: GenericArgs) -> AliasTerm < I > { interner . debug_assert_args_compatible (def_id , args) ; AliasTerm { def_id , args , _use_alias_term_new_instead : () } } pub fn new (interner : I , def_id : I :: DefId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> AliasTerm < I > { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , def_id , args) } pub fn expect_ty (self , interner : I) -> ty :: AliasTy < I > { match self . kind (interner) { AliasTermKind :: ProjectionTy | AliasTermKind :: InherentTy | AliasTermKind :: OpaqueTy | AliasTermKind :: FreeTy => { } AliasTermKind :: InherentConst | AliasTermKind :: FreeConst | AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst => { panic ! ("Cannot turn `UnevaluatedConst` into `AliasTy`") } } ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } } pub fn kind (self , interner : I) -> AliasTermKind { interner . alias_term_kind (self) } pub fn to_term (self , interner : I) -> I :: Term { match self . kind (interner) { AliasTermKind :: ProjectionTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Projection , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: InherentTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Inherent , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: OpaqueTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Opaque , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: FreeTy => Ty :: new_alias (interner , ty :: AliasTyKind :: Free , ty :: AliasTy { def_id : self . def_id , args : self . args , _use_alias_ty_new_instead : () } ,) . into () , AliasTermKind :: FreeConst | AliasTermKind :: InherentConst | AliasTermKind :: UnevaluatedConst | AliasTermKind :: ProjectionConst => I :: Const :: new_unevaluated (interner , ty :: UnevaluatedConst :: new (self . def_id , self . args) ,) . into () , } } }
    };
}

impl_376!();