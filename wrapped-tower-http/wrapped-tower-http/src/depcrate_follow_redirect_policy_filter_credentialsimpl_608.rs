// Generated macro for impl_608 (impl)
macro_rules! Depcrate_follow_redirect_policy_filter_credentialsimpl_608 {
() => {
// Module: crate::follow_redirect::policy::filter_credentials
// Provides: {"impl_608"}
// Dependencies: {}
impl FilterCredentials { # [doc = " Create a new [`FilterCredentials`] that removes blocklisted request headers in cross-origin"] # [doc = " redirections."] pub fn new () -> Self { FilterCredentials { block_cross_origin : true , block_any : false , remove_blocklisted : true , remove_all : false , blocked : false , } } # [doc = " Configure `self` to mark cross-origin redirections as \"blocked\"."] pub fn block_cross_origin (mut self , enable : bool) -> Self { self . block_cross_origin = enable ; self } # [doc = " Configure `self` to mark every redirection as \"blocked\"."] pub fn block_any (mut self) -> Self { self . block_any = true ; self } # [doc = " Configure `self` to mark no redirections as \"blocked\"."] pub fn block_none (mut self) -> Self { self . block_any = false ; self . block_cross_origin (false) } # [doc = " Configure `self` to remove blocklisted headers in \"blocked\" redirections."] # [doc = ""] # [doc = " The blocklist includes the following headers:"] # [doc = ""] # [doc = " - `Authorization`"] # [doc = " - `Cookie`"] # [doc = " - `Proxy-Authorization`"] pub fn remove_blocklisted (mut self , enable : bool) -> Self { self . remove_blocklisted = enable ; self } # [doc = " Configure `self` to remove all headers in \"blocked\" redirections."] pub fn remove_all (mut self) -> Self { self . remove_all = true ; self } # [doc = " Configure `self` to remove no headers in \"blocked\" redirections."] pub fn remove_none (mut self) -> Self { self . remove_all = false ; self . remove_blocklisted (false) } }
};
}
