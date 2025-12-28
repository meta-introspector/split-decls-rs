macro_rules! deps {
    () => {
        PolyFnSig!();
        IntTy!();
        UintTy!();
        TyConst!();
        Region!();
        GenericArgs!();
        DynKind!();
        FloatTy!();
        Pattern!();
        Ty!();
        ExistentialPredicate!();
        Binder!();
    };
}

macro_rules! RigidTy {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum RigidTy { Bool , Char , Int (IntTy) , Uint (UintTy) , Float (FloatTy) , Adt (AdtDef , GenericArgs) , Foreign (ForeignDef) , Str , Array (Ty , TyConst) , Pat (Ty , Pattern) , Slice (Ty) , RawPtr (Ty , Mutability) , Ref (Region , Ty , Mutability) , FnDef (FnDef , GenericArgs) , FnPtr (PolyFnSig) , Closure (ClosureDef , GenericArgs) , Coroutine (CoroutineDef , GenericArgs) , CoroutineClosure (CoroutineClosureDef , GenericArgs) , Dynamic (Vec < Binder < ExistentialPredicate > > , Region , DynKind) , Never , Tuple (Vec < Ty >) , CoroutineWitness (CoroutineWitnessDef , GenericArgs) , }
    };
}

RigidTy!();