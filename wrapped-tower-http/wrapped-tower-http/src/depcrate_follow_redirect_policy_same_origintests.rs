// Generated macro for tests (module)
macro_rules! Depcrate_follow_redirect_policy_same_origintests {
() => {
// Module: crate::follow_redirect::policy::same_origin
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use http :: { Request , Uri } ; # [test] fn works () { let mut policy = SameOrigin :: default () ; let initial = Uri :: from_static ("http://example.com/old") ; let same_origin = Uri :: from_static ("http://example.com/new") ; let cross_origin = Uri :: from_static ("https://example.com/new") ; let mut request = Request :: builder () . uri (initial) . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; let attempt = Attempt { status : Default :: default () , location : & same_origin , previous : request . uri () , } ; assert ! (Policy ::< () , () >:: redirect (& mut policy , & attempt) . unwrap () . is_follow ()) ; let mut request = Request :: builder () . uri (same_origin) . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; let attempt = Attempt { status : Default :: default () , location : & cross_origin , previous : request . uri () , } ; assert ! (Policy ::< () , () >:: redirect (& mut policy , & attempt) . unwrap () . is_stop ()) ; } }
};
}
