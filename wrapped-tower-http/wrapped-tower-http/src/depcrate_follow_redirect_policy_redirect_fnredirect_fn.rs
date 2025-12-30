// Generated macro for redirect_fn (function)
macro_rules! Depcrate_follow_redirect_policy_redirect_fnredirect_fn {
() => {
// Module: crate::follow_redirect::policy::redirect_fn
// Provides: {"redirect_fn"}
// Dependencies: {}
# [doc = " Create a new redirection [`Policy`] from a closure"] # [doc = " `F: FnMut(&Attempt<'_>) -> Result<Action, E>`."] # [doc = ""] # [doc = " [`redirect`][Policy::redirect] method of the returned `Policy` delegates to"] # [doc = " the wrapped closure."] pub fn redirect_fn < F , E > (f : F) -> RedirectFn < F > where F : FnMut (& Attempt < '_ >) -> Result < Action , E > , { RedirectFn { f } }
};
}
