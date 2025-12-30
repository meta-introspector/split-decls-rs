// Generated macro for replace_all_git_hub_actions_impl (function)
macro_rules! Depcrate_macros_replace_all_git_hub_actionsreplace_all_git_hub_actions_impl {
() => {
// Module: crate::macros::replace_all_git_hub_actions
// Provides: {"replace_all_git_hub_actions_impl"}
// Dependencies: {}
# [decl (fn , name = "replace_all_git_hub_actions_impl" , vis = "pub" , hash = "613ea2f0")] pub fn replace_all_git_hub_actions_impl (input : TokenStream) -> TokenStream { let new_workflow = parse_macro_input ! (input as LitStr) ; let span = new_workflow . span () ; quote_spanned ! { span => eprintln ! ("\n♻️ REPLACE ALL GITHUB ACTIONS! Conceptually replacing with: {}\n" , # new_workflow) ; () } . into () }
};
}
