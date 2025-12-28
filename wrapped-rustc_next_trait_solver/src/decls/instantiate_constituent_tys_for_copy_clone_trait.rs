macro_rules! deps {
    () => {
        EvalCtxt!();
        SolverDelegate!();
    };
}

macro_rules! instantiate_constituent_tys_for_copy_clone_trait {
    () => {
        deps!();
        # [instrument (level = "trace" , skip (ecx) , ret)] pub (in crate :: solve) fn instantiate_constituent_tys_for_copy_clone_trait < D , I > (ecx : & EvalCtxt < '_ , D > , ty : I :: Ty ,) -> Result < ty :: Binder < I , Vec < I :: Ty > > , NoSolution > where D : SolverDelegate < Interner = I > , I : Interner , { match ty . kind () { ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Error (_) => Ok (ty :: Binder :: dummy (vec ! [])) , ty :: Uint (_) | ty :: Int (_) | ty :: Infer (ty :: IntVar (_) | ty :: FloatVar (_)) | ty :: Bool | ty :: Float (_) | ty :: Char | ty :: RawPtr (..) | ty :: Never | ty :: Ref (_ , _ , Mutability :: Not) | ty :: Array (..) => Err (NoSolution) , ty :: Pat (ty , ..) => Ok (ty :: Binder :: dummy (vec ! [ty])) , ty :: Dynamic (..) | ty :: Str | ty :: Slice (_) | ty :: Foreign (..) | ty :: Ref (_ , _ , Mutability :: Mut) | ty :: Adt (_ , _) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Placeholder (..) => Err (NoSolution) , ty :: Bound (..) | ty :: Infer (ty :: TyVar (_) | ty :: FreshTy (_) | ty :: FreshIntTy (_) | ty :: FreshFloatTy (_)) => { panic ! ("unexpected type `{ty:?}`") } ty :: Tuple (tys) => Ok (ty :: Binder :: dummy (tys . to_vec ())) , ty :: Closure (_ , args) => Ok (ty :: Binder :: dummy (vec ! [args . as_closure () . tupled_upvars_ty ()])) , ty :: CoroutineClosure (_ , args) => { Ok (ty :: Binder :: dummy (vec ! [args . as_coroutine_closure () . tupled_upvars_ty ()])) } ty :: Coroutine (def_id , args) => match ecx . cx () . coroutine_movability (def_id) { Movability :: Static => Err (NoSolution) , Movability :: Movable => { if ecx . cx () . features () . coroutine_clone () { Ok (ty :: Binder :: dummy (vec ! [args . as_coroutine () . tupled_upvars_ty () , Ty :: new_coroutine_witness_for_coroutine (ecx . cx () , def_id , args) ,])) } else { Err (NoSolution) } } } , ty :: UnsafeBinder (_) => Err (NoSolution) , ty :: CoroutineWitness (def_id , args) => Ok (ecx . cx () . coroutine_hidden_types (def_id) . instantiate (ecx . cx () , args) . map_bound (| bound | bound . types . to_vec ())) , } }
    };
}

instantiate_constituent_tys_for_copy_clone_trait!()