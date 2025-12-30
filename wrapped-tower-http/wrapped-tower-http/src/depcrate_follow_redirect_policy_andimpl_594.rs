// Generated macro for impl_594 (impl)
macro_rules! Depcrate_follow_redirect_policy_andimpl_594 {
() => {
// Module: crate::follow_redirect::policy::and
// Provides: {"impl_594"}
// Dependencies: {}
impl < Bd , E , A , B > Policy < Bd , E > for And < A , B > where A : Policy < Bd , E > , B : Policy < Bd , E > , { fn redirect (& mut self , attempt : & Attempt < '_ >) -> Result < Action , E > { match self . a . redirect (attempt) { Ok (Action :: Follow) => self . b . redirect (attempt) , a => a , } } fn on_request (& mut self , request : & mut Request < Bd >) { self . a . on_request (request) ; self . b . on_request (request) ; } fn clone_body (& self , body : & Bd) -> Option < Bd > { self . a . clone_body (body) . or_else (| | self . b . clone_body (body)) } }
};
}
