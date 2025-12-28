macro_rules! deps {
    () => {
        SolverDelegate!();
        EvalCtxt!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < D , I > EvalCtxt < '_ , D > where D : SolverDelegate < Interner = I > , I : Interner , { fn translate_args (& mut self , goal : Goal < I , ty :: NormalizesTo < I > > , impl_def_id : I :: ImplId , impl_args : I :: GenericArgs , impl_trait_ref : rustc_type_ir :: TraitRef < I > , target_container_def_id : I :: DefId ,) -> Result < I :: GenericArgs , NoSolution > { let cx = self . cx () ; Ok (if target_container_def_id == impl_trait_ref . def_id . into () { goal . predicate . alias . args } else if target_container_def_id == impl_def_id . into () { goal . predicate . alias . args . rebase_onto (cx , impl_trait_ref . def_id . into () , impl_args) } else { let target_args = self . fresh_args_for_item (target_container_def_id) ; let target_trait_ref = cx . impl_trait_ref (target_container_def_id . try_into () . unwrap ()) . instantiate (cx , target_args) ; self . eq (goal . param_env , impl_trait_ref , target_trait_ref) ? ; self . add_goals (GoalSource :: Misc , cx . predicates_of (target_container_def_id) . iter_instantiated (cx , target_args) . map (| pred | goal . with (cx , pred)) ,) ; goal . predicate . alias . args . rebase_onto (cx , impl_trait_ref . def_id . into () , target_args) }) } }
    };
}

impl_118!()