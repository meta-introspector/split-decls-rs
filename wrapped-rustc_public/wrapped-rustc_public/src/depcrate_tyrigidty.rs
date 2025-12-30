// Generated macro for RigidTy (enum)
macro_rules! Depcrate_tyRigidTy {
() => {
// Module: crate::ty
// Provides: {"RigidTy"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum RigidTy { Bool , Char , Int (IntTy) , Uint (UintTy) , Float (FloatTy) , Adt (AdtDef , GenericArgs) , Foreign (ForeignDef) , Str , Array (Ty , TyConst) , Pat (Ty , Pattern) , Slice (Ty) , RawPtr (Ty , Mutability) , Ref (Region , Ty , Mutability) , FnDef (FnDef , GenericArgs) , FnPtr (PolyFnSig) , Closure (ClosureDef , GenericArgs) , Coroutine (CoroutineDef , GenericArgs) , CoroutineClosure (CoroutineClosureDef , GenericArgs) , Dynamic (Vec < Binder < ExistentialPredicate > > , Region , DynKind) , Never , Tuple (Vec < Ty >) , CoroutineWitness (CoroutineWitnessDef , GenericArgs) , }
};
}
