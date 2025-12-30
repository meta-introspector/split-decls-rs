// Generated macro for impl_679 (impl)
macro_rules! Depcrate_follow_redirectimpl_679 {
() => {
// Module: crate::follow_redirect
// Provides: {"impl_679"}
// Dependencies: {}
impl < B > BodyRepr < B > where B : Body + Default , { fn take (& mut self) -> Option < B > { match mem :: replace (self , BodyRepr :: None) { BodyRepr :: Some (body) => Some (body) , BodyRepr :: Empty => { * self = BodyRepr :: Empty ; Some (B :: default ()) } BodyRepr :: None => None , } } fn try_clone_from < P , E > (& mut self , body : & B , policy : & P) where P : Policy < B , E > , { match self { BodyRepr :: Some (_) | BodyRepr :: Empty => { } BodyRepr :: None => { if let Some (body) = clone_body (policy , body) { * self = BodyRepr :: Some (body) ; } } } } }
};
}
