macro_rules! deps {
    () => {
        GenericArg!();
        Interner!();
        TraitRef!();
        GenericArgs!();
        ExistentialTraitRef!();
        Ty!();
    };
}

macro_rules! impl_365 {
    () => {
        deps!();
        impl < I : Interner > ExistentialTraitRef < I > { pub fn new_from_args (interner : I , trait_def_id : I :: TraitId , args : I :: GenericArgs) -> Self { interner . debug_assert_existential_args_compatible (trait_def_id . into () , args) ; Self { def_id : trait_def_id , args , _use_existential_trait_ref_new_instead : () } } pub fn new (interner : I , trait_def_id : I :: TraitId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> Self { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , trait_def_id , args) } pub fn erase_self_ty (interner : I , trait_ref : TraitRef < I >) -> ExistentialTraitRef < I > { trait_ref . args . type_at (0) ; ExistentialTraitRef { def_id : trait_ref . def_id , args : interner . mk_args (& trait_ref . args . as_slice () [1 ..]) , _use_existential_trait_ref_new_instead : () , } } # [doc = " Object types don't have a self type specified. Therefore, when"] # [doc = " we convert the principal trait-ref into a normal trait-ref,"] # [doc = " you must give *some* self type. A common choice is `mk_err()`"] # [doc = " or some placeholder type."] pub fn with_self_ty (self , interner : I , self_ty : I :: Ty) -> TraitRef < I > { TraitRef :: new (interner , self . def_id , [self_ty . into ()] . into_iter () . chain (self . args . iter ())) } }
    };
}

impl_365!()