// Generated macro for impl_657 (impl)
macro_rules! Depcrate_ty_kindimpl_657 {
() => {
// Module: crate::ty_kind
// Provides: {"impl_657"}
// Dependencies: {}
impl UnifyValue for FloatVarValue { type Error = NoError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , Self :: Error > { match (* value1 , * value2) { (FloatVarValue :: Unknown , FloatVarValue :: Unknown) => Ok (FloatVarValue :: Unknown) , (FloatVarValue :: Unknown , FloatVarValue :: Known (known)) | (FloatVarValue :: Known (known) , FloatVarValue :: Unknown) => { Ok (FloatVarValue :: Known (known)) } (FloatVarValue :: Known (_) , FloatVarValue :: Known (_)) => { panic ! ("differing floats should have been resolved first") } } } }
};
}
