// Generated macro for GenericArgs (trait)
macro_rules! Depcrate_inherentGenericArgs {
() => {
// Module: crate::inherent
// Provides: {"GenericArgs"}
// Dependencies: {}
pub trait GenericArgs < I : Interner < GenericArgs = Self > > : Copy + Debug + Hash + Eq + SliceLike < Item = I :: GenericArg > + Default + Relate < I > { fn rebase_onto (self , interner : I , source_def_id : I :: DefId , target : I :: GenericArgs ,) -> I :: GenericArgs ; fn type_at (self , i : usize) -> I :: Ty ; fn region_at (self , i : usize) -> I :: Region ; fn const_at (self , i : usize) -> I :: Const ; fn identity_for_item (interner : I , def_id : I :: DefId) -> I :: GenericArgs ; fn extend_with_error (interner : I , def_id : I :: DefId , original_args : & [I :: GenericArg] ,) -> I :: GenericArgs ; fn split_closure_args (self) -> ty :: ClosureArgsParts < I > ; fn split_coroutine_closure_args (self) -> ty :: CoroutineClosureArgsParts < I > ; fn split_coroutine_args (self) -> ty :: CoroutineArgsParts < I > ; fn as_closure (self) -> ty :: ClosureArgs < I > { ty :: ClosureArgs { args : self } } fn as_coroutine_closure (self) -> ty :: CoroutineClosureArgs < I > { ty :: CoroutineClosureArgs { args : self } } fn as_coroutine (self) -> ty :: CoroutineArgs < I > { ty :: CoroutineArgs { args : self } } }
};
}
