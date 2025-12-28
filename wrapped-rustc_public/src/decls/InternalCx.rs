macro_rules! deps {
    () => {
        DefId!();
        RustcInternal!();
        Ty!();
        BoundVariableKind!();
        Pattern!();
        Region!();
    };
}

macro_rules! InternalCx {
    () => {
        deps!();
        # [doc = " Trait that defines the methods that are fine to call from [`RustcInternal`]."] # [doc = ""] # [doc = " This trait is only for [`RustcInternal`]. Any other other access to rustc's internals"] # [doc = " should go through [`rustc_public_bridge::context::CompilerCtxt`]."] pub trait InternalCx < 'tcx > : Copy + Clone { fn tcx (self) -> TyCtxt < 'tcx > ; fn lift < T : ty :: Lift < TyCtxt < 'tcx > > > (self , value : T) -> Option < T :: Lifted > ; fn mk_args_from_iter < I , T > (self , iter : I) -> T :: Output where I : Iterator < Item = T > , T : ty :: CollectAndApply < ty :: GenericArg < 'tcx > , ty :: GenericArgsRef < 'tcx > > ; fn mk_pat (self , v : ty :: PatternKind < 'tcx >) -> ty :: Pattern < 'tcx > ; fn mk_poly_existential_predicates (self , eps : & [ty :: PolyExistentialPredicate < 'tcx >] ,) -> & 'tcx List < ty :: PolyExistentialPredicate < 'tcx > > ; fn mk_type_list (self , v : & [Ty < 'tcx >]) -> & 'tcx List < Ty < 'tcx > > ; fn lifetimes_re_erased (self) -> ty :: Region < 'tcx > ; fn mk_bound_variable_kinds_from_iter < I , T > (self , iter : I) -> T :: Output where I : Iterator < Item = T > , T : ty :: CollectAndApply < ty :: BoundVariableKind , & 'tcx List < ty :: BoundVariableKind > > ; fn mk_place_elems (self , v : & [mir :: PlaceElem < 'tcx >]) -> & 'tcx List < mir :: PlaceElem < 'tcx > > ; fn adt_def (self , def_id : rustc_hir :: def_id :: DefId) -> ty :: AdtDef < 'tcx > ; }
    };
}

InternalCx!();