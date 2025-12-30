// Generated macro for shared_ref (function)
macro_rules! Depcrate_utilshared_ref {
() => {
// Module: crate::util
// Provides: {"shared_ref"}
// Dependencies: {}
# [doc = " Take a type and create an immutable shared reference to that type."] pub (crate) fn shared_ref (ty : syn :: Type , mutable : bool) -> syn :: Type { syn :: TypeReference { and_token : Default :: default () , lifetime : None , mutability : if mutable { Some (syn :: token :: Mut :: default ()) } else { None } , elem : Box :: new (ty) , } . into () }
};
}
