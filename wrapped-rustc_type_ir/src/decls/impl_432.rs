macro_rules! deps {
    () => {
        Safety!();
        Interner!();
        Abi!();
        TyKind!();
        Binder!();
        FnSig!();
    };
}

macro_rules! impl_432 {
    () => {
        deps!();
        impl < I : Interner > TyKind < I > { pub fn fn_sig (self , interner : I) -> ty :: Binder < I , ty :: FnSig < I > > { match self { ty :: FnPtr (sig_tys , hdr) => sig_tys . with (hdr) , ty :: FnDef (def_id , args) => interner . fn_sig (def_id) . instantiate (interner , args) , ty :: Error (_) => { ty :: Binder :: dummy (ty :: FnSig { inputs_and_output : Default :: default () , c_variadic : false , safety : I :: Safety :: safe () , abi : I :: Abi :: rust () , }) } ty :: Closure (..) => panic ! ("to get the signature of a closure, use `args.as_closure().sig()` not `fn_sig()`" ,) , _ => panic ! ("Ty::fn_sig() called on non-fn type: {:?}" , self) , } } # [doc = " Returns `true` when the outermost type cannot be further normalized,"] # [doc = " resolved, or instantiated. This includes all primitive types, but also"] # [doc = " things like ADTs and trait objects, since even if their arguments or"] # [doc = " nested types may be further simplified, the outermost [`ty::TyKind`] or"] # [doc = " type constructor remains the same."] pub fn is_known_rigid (self) -> bool { match self { ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Adt (_ , _) | ty :: Foreign (_) | ty :: Str | ty :: Array (_ , _) | ty :: Pat (_ , _) | ty :: Slice (_) | ty :: RawPtr (_ , _) | ty :: Ref (_ , _ , _) | ty :: FnDef (_ , _) | ty :: FnPtr (..) | ty :: UnsafeBinder (_) | ty :: Dynamic (_ , _ , _) | ty :: Closure (_ , _) | ty :: CoroutineClosure (_ , _) | ty :: Coroutine (_ , _) | ty :: CoroutineWitness (..) | ty :: Never | ty :: Tuple (_) => true , ty :: Error (_) | ty :: Infer (_) | ty :: Alias (_ , _) | ty :: Param (_) | ty :: Bound (_ , _) | ty :: Placeholder (_) => false , } } }
    };
}

impl_432!()