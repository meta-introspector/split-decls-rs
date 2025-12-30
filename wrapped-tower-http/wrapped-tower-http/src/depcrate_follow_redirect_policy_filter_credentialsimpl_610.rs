// Generated macro for impl_610 (impl)
macro_rules! Depcrate_follow_redirect_policy_filter_credentialsimpl_610 {
() => {
// Module: crate::follow_redirect::policy::filter_credentials
// Provides: {"impl_610"}
// Dependencies: {}
impl < B , E > Policy < B , E > for FilterCredentials { fn redirect (& mut self , attempt : & Attempt < '_ >) -> Result < Action , E > { self . blocked = self . block_any || (self . block_cross_origin && ! eq_origin (attempt . previous () , attempt . location ())) ; Ok (Action :: Follow) } fn on_request (& mut self , request : & mut Request < B >) { if self . blocked { let headers = request . headers_mut () ; if self . remove_all { headers . clear () ; } else if self . remove_blocklisted { for key in BLOCKLIST { headers . remove (key) ; } } } } }
};
}
