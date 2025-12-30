// Generated macro for impl_655 (impl)
macro_rules! Depcrate_ty_kindimpl_655 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_655"}
// Dependencies: {}
impl UnifyValue for IntVarValue { type Error = NoError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , Self :: Error > { match (* value1 , * value2) { (IntVarValue :: Unknown , IntVarValue :: Unknown) => Ok (IntVarValue :: Unknown) , (IntVarValue :: Unknown , known @ (IntVarValue :: UintType (_) | IntVarValue :: IntType (_)) ,) | (known @ (IntVarValue :: UintType (_) | IntVarValue :: IntType (_)) , IntVarValue :: Unknown ,) => Ok (known) , _ => panic ! ("differing ints should have been resolved first") , } } }
};
}
