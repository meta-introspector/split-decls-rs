// Generated macro for tests (module)
macro_rules! Depcrate_follow_redirect_policy_limitedtests {
() => {
// Module: crate::follow_redirect::policy::limited
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use http :: { Request , Uri } ; use super :: * ; # [test] fn works () { let uri = Uri :: from_static ("https://example.com/") ; let mut policy = Limited :: new (2) ; for _ in 0 .. 2 { let mut request = Request :: builder () . uri (uri . clone ()) . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; let attempt = Attempt { status : Default :: default () , location : & uri , previous : & uri , } ; assert ! (Policy ::< () , () >:: redirect (& mut policy , & attempt) . unwrap () . is_follow ()) ; } let mut request = Request :: builder () . uri (uri . clone ()) . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; let attempt = Attempt { status : Default :: default () , location : & uri , previous : & uri , } ; assert ! (Policy ::< () , () >:: redirect (& mut policy , & attempt) . unwrap () . is_stop ()) ; } }
};
}
