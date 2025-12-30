// Generated macro for impl_624 (impl)
macro_rules! Depcrate_follow_redirect_policy_orimpl_624 {
() => {
// Module: crate::follow_redirect::policy::or
// Provides: {"impl_624"}
// Dependencies: {}
impl < Bd , E , A , B > Policy < Bd , E > for Or < A , B > where A : Policy < Bd , E > , B : Policy < Bd , E > , { fn redirect (& mut self , attempt : & Attempt < '_ >) -> Result < Action , E > { match self . a . redirect (attempt) { Ok (Action :: Stop) | Err (_) => self . b . redirect (attempt) , a => a , } } fn on_request (& mut self , request : & mut Request < Bd >) { self . a . on_request (request) ; self . b . on_request (request) ; } fn clone_body (& self , body : & Bd) -> Option < Bd > { self . a . clone_body (body) . or_else (| | self . b . clone_body (body)) } }
};
}
