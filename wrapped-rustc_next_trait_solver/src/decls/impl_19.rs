macro_rules! deps {
    () => {
        OrphanCheckEarlyExit!();
        OrphanChecker!();
        InCrate!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'a , Infcx , I , F , E > OrphanChecker < 'a , Infcx , I , F > where Infcx : InferCtxtLike < Interner = I > , I : Interner , F : FnOnce (I :: Ty) -> Result < I :: Ty , E > , { fn new (infcx : & 'a Infcx , in_crate : InCrate , lazily_normalize_ty : F) -> Self { OrphanChecker { infcx , in_crate , in_self_ty : true , lazily_normalize_ty , search_first_local_ty : false , non_local_tys : Vec :: new () , } } fn found_non_local_ty (& mut self , t : I :: Ty) -> ControlFlow < OrphanCheckEarlyExit < I , E > > { self . non_local_tys . push ((t , self . in_self_ty . into ())) ; ControlFlow :: Continue (()) } fn found_uncovered_ty_param (& mut self , ty : I :: Ty) -> ControlFlow < OrphanCheckEarlyExit < I , E > > { if self . search_first_local_ty { return ControlFlow :: Continue (()) ; } ControlFlow :: Break (OrphanCheckEarlyExit :: UncoveredTyParam (ty)) } fn def_id_is_local (& mut self , def_id : impl DefId < I >) -> bool { match self . in_crate { InCrate :: Local { .. } => def_id . is_local () , InCrate :: Remote => false , } } }
    };
}

impl_19!()