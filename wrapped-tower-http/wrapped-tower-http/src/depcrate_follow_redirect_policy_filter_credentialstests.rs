// Generated macro for tests (module)
macro_rules! Depcrate_follow_redirect_policy_filter_credentialstests {
() => {
// Module: crate::follow_redirect::policy::filter_credentials
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use http :: Uri ; # [test] fn works () { let mut policy = FilterCredentials :: default () ; let initial = Uri :: from_static ("http://example.com/old") ; let same_origin = Uri :: from_static ("http://example.com/new") ; let cross_origin = Uri :: from_static ("https://example.com/new") ; let mut request = Request :: builder () . uri (initial) . header (header :: COOKIE , "42") . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; assert ! (request . headers () . contains_key (header :: COOKIE)) ; let attempt = Attempt { status : Default :: default () , location : & same_origin , previous : request . uri () , } ; assert ! (Policy ::< () , () >:: redirect (& mut policy , & attempt) . unwrap () . is_follow ()) ; let mut request = Request :: builder () . uri (same_origin) . header (header :: COOKIE , "42") . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; assert ! (request . headers () . contains_key (header :: COOKIE)) ; let attempt = Attempt { status : Default :: default () , location : & cross_origin , previous : request . uri () , } ; assert ! (Policy ::< () , () >:: redirect (& mut policy , & attempt) . unwrap () . is_follow ()) ; let mut request = Request :: builder () . uri (cross_origin) . header (header :: COOKIE , "42") . body (()) . unwrap () ; Policy :: < () , () > :: on_request (& mut policy , & mut request) ; assert ! (! request . headers () . contains_key (header :: COOKIE)) ; } }
};
}
