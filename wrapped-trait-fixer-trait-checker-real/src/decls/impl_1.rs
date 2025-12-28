macro_rules! deps {
    () => {
        RustcTyCtxt!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx > TraitChecker < 'tcx , TyCtxt < 'tcx > , DefId , Ty < 'tcx > > for RustcTyCtxt < 'tcx > { fn get_trait_def_id (& self , trait_name : & str) -> Option < DefId > { match trait_name { "Clone" => self . 0 . lang_items () . clone_trait () , "Debug" => self . 0 . get_diagnostic_item (Symbol :: intern ("Debug")) , _ => { let trait_sym = Symbol :: intern (trait_name) ; self . 0 . get_diagnostic_item (trait_sym) } } } fn type_implements_trait (& self , _tcx_param : TyCtxt < 'tcx > , adt_ty : Ty < 'tcx > , _item_def_id : DefId , trait_def_id : DefId ,) -> bool { let tcx = self . 0 ; let infcx = tcx . infer_ctxt () . build (TypingMode :: Analysis { defining_opaque_types_and_generators : Default :: default () }) ; let param_env = ParamEnv :: empty () ; let predicates = [Obligation { cause : ObligationCause :: new (DUMMY_SP , DefId :: local (DefIndex :: from_usize (0)) . expect_local () , ObligationCauseCode :: Misc) , param_env , predicate : tcx . mk_predicate (ty :: Binder :: dummy (ty :: PredicateKind :: Clause (ty :: ClauseKind :: Trait (ty :: TraitPredicate { trait_ref : ty :: TraitRef :: new (tcx , trait_def_id , tcx . mk_args (& [adt_ty . into ()])) , polarity : ty :: PredicatePolarity :: Positive , })))) , recursion_depth : 0 , }] ; let mut fulfill_cx : FulfillmentContext < '_ , FulfillmentError < 'tcx > > = FulfillmentContext :: new (& infcx) ; for predicate in predicates { fulfill_cx . register_predicate_obligation (& infcx , predicate) ; } let errors = fulfill_cx . try_evaluate_obligations (& infcx) ; errors . is_empty () } }
    };
}

impl_1!();