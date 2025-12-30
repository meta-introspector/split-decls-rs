// Generated macro for impl_43 (impl)
macro_rules! Depcrate_assertimpl_43 {
() => {
// Module: crate::assert
// Provides: {"impl_43"}
// Dependencies: {}
# [doc = " # Customize Behavior"] impl Assert { # [doc = " Override the color palette"] pub fn palette (mut self , palette : crate :: report :: Palette) -> Self { self . palette = palette ; self } # [doc = " Read the failure action from an environment variable"] pub fn action_env (mut self , var_name : & str) -> Self { let action = Action :: with_env_var (var_name) ; self . action = action . unwrap_or (self . action) ; self . action_var = Some (var_name . to_owned ()) ; self } # [doc = " Override the failure action"] pub fn action (mut self , action : Action) -> Self { self . action = action ; self . action_var = None ; self } # [doc = " Override the default [`Redactions`][crate::Redactions]"] pub fn redact_with (mut self , substitutions : crate :: Redactions) -> Self { self . substitutions = substitutions ; self } # [doc = " Override the default [`Redactions`][crate::Redactions]"] # [deprecated (since = "0.6.2" , note = "Replaced with `Assert::redact_with`")] pub fn substitutions (self , substitutions : crate :: Redactions) -> Self { self . redact_with (substitutions) } # [doc = " Specify whether text should have path separators normalized"] # [doc = ""] # [doc = " The default is normalized"] pub fn normalize_paths (mut self , yes : bool) -> Self { self . normalize_paths = yes ; self } }
};
}
