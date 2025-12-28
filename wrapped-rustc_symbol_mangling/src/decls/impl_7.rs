macro_rules! deps {
    () => {
        AbiHashStable!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < 'tcx > AbiHashStable < 'tcx > for Ty < 'tcx > { fn abi_hash (& self , tcx : TyCtxt < 'tcx > , hasher : & mut StableHasher) { match self . kind () { ty :: Bool => sym :: bool . abi_hash (tcx , hasher) , ty :: Char => sym :: char . abi_hash (tcx , hasher) , ty :: Int (int_ty) => int_ty . name_str () . abi_hash (tcx , hasher) , ty :: Uint (uint_ty) => uint_ty . name_str () . abi_hash (tcx , hasher) , ty :: Float (float_ty) => float_ty . name_str () . abi_hash (tcx , hasher) , ty :: Adt (adt_def , args) => { adt_def . is_struct () . abi_hash (tcx , hasher) ; adt_def . is_enum () . abi_hash (tcx , hasher) ; adt_def . is_union () . abi_hash (tcx , hasher) ; if let Some (align) = adt_def . repr () . align { align . bits () . abi_hash (tcx , hasher) ; } if let Some (integer) = adt_def . repr () . int { match integer { IntegerType :: Pointer (sign) => sign . abi_hash (tcx , hasher) , IntegerType :: Fixed (integer , sign) => { integer . int_ty_str () . abi_hash (tcx , hasher) ; sign . abi_hash (tcx , hasher) ; } } } if let Some (pack) = adt_def . repr () . pack { pack . bits () . abi_hash (tcx , hasher) ; } adt_def . repr () . c () . abi_hash (tcx , hasher) ; for variant in adt_def . variants () { variant . name . abi_hash (tcx , hasher) ; for field in & variant . fields { field . name . abi_hash (tcx , hasher) ; let field_ty = tcx . type_of (field . did) . instantiate_identity () ; field_ty . abi_hash (tcx , hasher) ; } } args . abi_hash (tcx , hasher) ; } ty :: Tuple (args) if args . len () == 0 => { } ty :: Foreign (_) | ty :: Ref (_ , _ , _) | ty :: Str | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (_ , _) | ty :: Dynamic (_ , _ , _) | ty :: Closure (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (_ , _) | ty :: Never | ty :: Tuple (_) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Bound (_ , _) | ty :: Placeholder (_) | ty :: Infer (_) | ty :: UnsafeBinder (_) => unreachable ! () , ty :: Error (_) => { } } } }
    };
}

impl_7!();