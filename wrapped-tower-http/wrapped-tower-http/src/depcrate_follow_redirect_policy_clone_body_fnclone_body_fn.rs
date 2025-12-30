// Generated macro for clone_body_fn (function)
macro_rules! Depcrate_follow_redirect_policy_clone_body_fnclone_body_fn {
() => {
// Module: crate::follow_redirect::policy::clone_body_fn
// Provides: {"clone_body_fn"}
// Dependencies: {}
# [doc = " Create a new redirection [`Policy`] from a closure `F: Fn(&B) -> Option<B>`."] # [doc = ""] # [doc = " [`clone_body`][Policy::clone_body] method of the returned `Policy` delegates to the wrapped"] # [doc = " closure and [`redirect`][Policy::redirect] method always returns [`Action::Follow`]."] pub fn clone_body_fn < F , B > (f : F) -> CloneBodyFn < F > where F : Fn (& B) -> Option < B > , { CloneBodyFn { f } }
};
}
