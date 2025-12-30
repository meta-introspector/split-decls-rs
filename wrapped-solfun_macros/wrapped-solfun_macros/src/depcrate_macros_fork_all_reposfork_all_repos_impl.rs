// Generated macro for fork_all_repos_impl (function)
macro_rules! Depcrate_macros_fork_all_reposfork_all_repos_impl {
() => {
// Module: crate::macros::fork_all_repos
// Provides: {"fork_all_repos_impl"}
// Dependencies: {}
# [decl (fn , name = "fork_all_repos_impl" , vis = "pub" , hash = "eaae29c6")] pub fn fork_all_repos_impl (input : TokenStream) -> TokenStream { let org_name = parse_macro_input ! (input as LitStr) ; let span = org_name . span () ; quote_spanned ! { span => eprintln ! ("\n🍴 FORK ALL REPOS! Conceptually forking all repos in organization: {}\n" , # org_name) ; () } . into () }
};
}
