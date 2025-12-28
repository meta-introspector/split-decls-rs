macro_rules! deps {
    () => {
        Interner!();
        GenericArgs!();
        TraitRef!();
        Ty!();
        GenericArg!();
    };
}

macro_rules! impl_345 {
    () => {
        deps!();
        impl < I : Interner > TraitRef < I > { pub fn new_from_args (interner : I , trait_def_id : I :: TraitId , args : I :: GenericArgs) -> Self { interner . debug_assert_args_compatible (trait_def_id . into () , args) ; Self { def_id : trait_def_id , args , _use_trait_ref_new_instead : () } } pub fn new (interner : I , trait_def_id : I :: TraitId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> Self { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , trait_def_id , args) } pub fn from_assoc (interner : I , trait_id : I :: TraitId , args : I :: GenericArgs) -> TraitRef < I > { let generics = interner . generics_of (trait_id . into ()) ; TraitRef :: new (interner , trait_id , args . iter () . take (generics . count ())) } # [doc = " Returns a `TraitRef` of the form `P0: Foo<P1..Pn>` where `Pi`"] # [doc = " are the parameters defined on trait."] pub fn identity (interner : I , def_id : I :: TraitId) -> TraitRef < I > { TraitRef :: new_from_args (interner , def_id , I :: GenericArgs :: identity_for_item (interner , def_id . into ()) ,) } pub fn with_replaced_self_ty (self , interner : I , self_ty : I :: Ty) -> Self { TraitRef :: new (interner , self . def_id , [self_ty . into ()] . into_iter () . chain (self . args . iter () . skip (1)) ,) } # [inline] pub fn self_ty (& self) -> I :: Ty { self . args . type_at (0) } }
    };
}

impl_345!();