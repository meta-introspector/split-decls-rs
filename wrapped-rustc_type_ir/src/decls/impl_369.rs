macro_rules! deps {
    () => {
        Ty!();
        GenericArgs!();
        ExistentialTraitRef!();
        Term!();
        ExistentialProjection!();
        ProjectionPredicate!();
        AliasTerm!();
        DefId!();
        GenericArg!();
        Interner!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl < I : Interner > ExistentialProjection < I > { pub fn new_from_args (interner : I , def_id : I :: DefId , args : I :: GenericArgs , term : I :: Term ,) -> ExistentialProjection < I > { interner . debug_assert_existential_args_compatible (def_id , args) ; Self { def_id , args , term , use_existential_projection_new_instead : () } } pub fn new (interner : I , def_id : I :: DefId , args : impl IntoIterator < Item : Into < I :: GenericArg > > , term : I :: Term ,) -> ExistentialProjection < I > { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , def_id , args , term) } # [doc = " Extracts the underlying existential trait reference from this projection."] # [doc = " For example, if this is a projection of `exists T. <T as Iterator>::Item == X`,"] # [doc = " then this function would return an `exists T. T: Iterator` existential trait"] # [doc = " reference."] pub fn trait_ref (& self , interner : I) -> ExistentialTraitRef < I > { let def_id = interner . parent (self . def_id) ; let args_count = interner . generics_of (def_id) . count () - 1 ; let args = interner . mk_args (& self . args . as_slice () [.. args_count]) ; ExistentialTraitRef :: new_from_args (interner , def_id . try_into () . unwrap () , args) } pub fn with_self_ty (& self , interner : I , self_ty : I :: Ty) -> ProjectionPredicate < I > { debug_assert ! (! self_ty . has_escaping_bound_vars ()) ; ProjectionPredicate { projection_term : AliasTerm :: new (interner , self . def_id , [self_ty . into ()] . iter () . chain (self . args . iter ()) ,) , term : self . term , } } pub fn erase_self_ty (interner : I , projection_predicate : ProjectionPredicate < I >) -> Self { projection_predicate . projection_term . args . type_at (0) ; Self { def_id : projection_predicate . projection_term . def_id , args : interner . mk_args (& projection_predicate . projection_term . args . as_slice () [1 ..]) , term : projection_predicate . term , use_existential_projection_new_instead : () , } } }
    };
}

impl_369!()