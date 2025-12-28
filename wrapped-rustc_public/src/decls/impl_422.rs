macro_rules! deps {
    () => {
        GenericArgs!();
        TraitRef!();
        GenericArgKind!();
        Ty!();
    };
}

macro_rules! impl_422 {
    () => {
        deps!();
        impl TraitRef { pub fn new (def_id : TraitDef , self_ty : Ty , gen_args : & GenericArgs) -> TraitRef { let mut args = vec ! [GenericArgKind :: Type (self_ty)] ; args . extend_from_slice (& gen_args . 0) ; TraitRef { def_id , args : GenericArgs (args) } } pub fn try_new (def_id : TraitDef , args : GenericArgs) -> Result < TraitRef , () > { match & args . 0 [..] { [GenericArgKind :: Type (_) , ..] => Ok (TraitRef { def_id , args }) , _ => Err (()) , } } pub fn args (& self) -> & GenericArgs { & self . args } pub fn self_ty (& self) -> Ty { let GenericArgKind :: Type (self_ty) = self . args . 0 [0] else { panic ! ("Self must be a type, but found: {:?}" , self . args . 0 [0]) } ; self_ty } }
    };
}

impl_422!()