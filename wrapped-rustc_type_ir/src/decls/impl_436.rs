macro_rules! deps {
    () => {
        Interner!();
        GenericArg!();
        AliasTy!();
        DefId!();
        AliasTyKind!();
        Ty!();
        GenericArgs!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        impl < I : Interner > AliasTy < I > { pub fn new_from_args (interner : I , def_id : I :: DefId , args : I :: GenericArgs) -> AliasTy < I > { interner . debug_assert_args_compatible (def_id , args) ; AliasTy { def_id , args , _use_alias_ty_new_instead : () } } pub fn new (interner : I , def_id : I :: DefId , args : impl IntoIterator < Item : Into < I :: GenericArg > > ,) -> AliasTy < I > { let args = interner . mk_args_from_iter (args . into_iter () . map (Into :: into)) ; Self :: new_from_args (interner , def_id , args) } pub fn kind (self , interner : I) -> AliasTyKind { interner . alias_ty_kind (self) } # [doc = " Whether this alias type is an opaque."] pub fn is_opaque (self , interner : I) -> bool { matches ! (self . kind (interner) , AliasTyKind :: Opaque) } pub fn to_ty (self , interner : I) -> I :: Ty { Ty :: new_alias (interner , self . kind (interner) , self) } }
    };
}

impl_436!();