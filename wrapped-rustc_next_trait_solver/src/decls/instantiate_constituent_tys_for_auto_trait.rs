macro_rules! deps {
    () => {
        SolverDelegate!();
        EvalCtxt!();
    };
}

macro_rules! instantiate_constituent_tys_for_auto_trait {
    () => {
        deps!();
        # [instrument (level = "trace" , skip (ecx) , ret)] pub (in crate :: solve) fn instantiate_constituent_tys_for_auto_trait < D , I > (ecx : & EvalCtxt < '_ , D > , ty : I :: Ty ,) -> Result < ty :: Binder < I , Vec < I :: Ty > > , NoSolution > where D : SolverDelegate < Interner = I > , I : Interner , { let cx = ecx . cx () ; match ty . kind () { ty :: Uint (_) | ty :: Int (_) | ty :: Bool | ty :: Float (_) | ty :: FnDef (..) | ty :: FnPtr (..) | ty :: Error (_) | ty :: Never | ty :: Char => Ok (ty :: Binder :: dummy (vec ! [])) , ty :: Foreign (..) => Ok (ty :: Binder :: dummy (vec ! [])) , ty :: Str => Ok (ty :: Binder :: dummy (vec ! [Ty :: new_slice (cx , Ty :: new_u8 (cx))])) , ty :: Dynamic (..) | ty :: Param (..) | ty :: Alias (ty :: Projection | ty :: Inherent | ty :: Free , ..) | ty :: Placeholder (..) | ty :: Bound (..) | ty :: Infer (_) => { panic ! ("unexpected type `{ty:?}`") } ty :: RawPtr (element_ty , _) | ty :: Ref (_ , element_ty , _) => { Ok (ty :: Binder :: dummy (vec ! [element_ty])) } ty :: Pat (element_ty , _) | ty :: Array (element_ty , _) | ty :: Slice (element_ty) => { Ok (ty :: Binder :: dummy (vec ! [element_ty])) } ty :: Tuple (tys) => { Ok (ty :: Binder :: dummy (tys . to_vec ())) } ty :: Closure (_ , args) => Ok (ty :: Binder :: dummy (vec ! [args . as_closure () . tupled_upvars_ty ()])) , ty :: CoroutineClosure (_ , args) => { Ok (ty :: Binder :: dummy (vec ! [args . as_coroutine_closure () . tupled_upvars_ty ()])) } ty :: Coroutine (def_id , args) => Ok (ty :: Binder :: dummy (vec ! [args . as_coroutine () . tupled_upvars_ty () , Ty :: new_coroutine_witness_for_coroutine (ecx . cx () , def_id , args) ,])) , ty :: CoroutineWitness (def_id , args) => Ok (ecx . cx () . coroutine_hidden_types (def_id) . instantiate (cx , args) . map_bound (| bound | bound . types . to_vec ())) , ty :: UnsafeBinder (bound_ty) => Ok (bound_ty . map_bound (| ty | vec ! [ty])) , ty :: Adt (def , args) if def . is_phantom_data () => Ok (ty :: Binder :: dummy (vec ! [args . type_at (0)])) , ty :: Adt (def , args) => { Ok (ty :: Binder :: dummy (def . all_field_tys (cx) . iter_instantiated (cx , args) . collect ())) } ty :: Alias (ty :: Opaque , ty :: AliasTy { def_id , args , .. }) => { Ok (ty :: Binder :: dummy (vec ! [cx . type_of (def_id) . instantiate (cx , args)])) } } }
    };
}

instantiate_constituent_tys_for_auto_trait!();