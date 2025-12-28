macro_rules! deps {
    () => {
        AsyncCallableRelevantTypes!();
    };
}

macro_rules! fn_item_to_async_callable {
    () => {
        deps!();
        fn fn_item_to_async_callable < I : Interner > (cx : I , bound_sig : ty :: Binder < I , ty :: FnSig < I > > ,) -> Result < (ty :: Binder < I , AsyncCallableRelevantTypes < I > > , Vec < I :: Predicate >) , NoSolution > { let sig = bound_sig . skip_binder () ; let future_trait_def_id = cx . require_trait_lang_item (SolverTraitLangItem :: Future) ; let nested = vec ! [bound_sig . rebind (ty :: TraitRef :: new (cx , future_trait_def_id , [sig . output ()])) . upcast (cx) ,] ; let future_output_def_id = cx . require_lang_item (SolverLangItem :: FutureOutput) ; let future_output_ty = Ty :: new_projection (cx , future_output_def_id , [sig . output ()]) ; Ok ((bound_sig . rebind (AsyncCallableRelevantTypes { tupled_inputs_ty : Ty :: new_tup (cx , sig . inputs () . as_slice ()) , output_coroutine_ty : sig . output () , coroutine_return_ty : future_output_ty , }) , nested ,)) }
    };
}

fn_item_to_async_callable!()