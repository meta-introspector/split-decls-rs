// Generated macro for tests (module)
macro_rules! Depcrate_follow_redirect_policytests {
() => {
// Module: crate::follow_redirect::policy
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn eq_origin_works () { assert ! (eq_origin (& Uri :: from_static ("https://example.com/1") , & Uri :: from_static ("https://example.com/2"))) ; assert ! (eq_origin (& Uri :: from_static ("https://example.com:443/") , & Uri :: from_static ("https://example.com/"))) ; assert ! (eq_origin (& Uri :: from_static ("https://example.com/") , & Uri :: from_static ("https://user@example.com/"))) ; assert ! (! eq_origin (& Uri :: from_static ("https://example.com/") , & Uri :: from_static ("https://www.example.com/"))) ; assert ! (! eq_origin (& Uri :: from_static ("https://example.com/") , & Uri :: from_static ("http://example.com/"))) ; } }
};
}
