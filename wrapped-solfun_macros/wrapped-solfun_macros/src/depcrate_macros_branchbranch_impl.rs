// Generated macro for branch_impl (function)
macro_rules! Depcrate_macros_branchbranch_impl {
() => {
// Module: crate::macros::branch
// Provides: {"branch_impl"}
// Dependencies: {}
# [decl (fn , name = "branch_impl" , vis = "pub" , hash = "70cfd19d")] pub fn branch_impl (input : TokenStream) -> TokenStream { let branch_name = parse_macro_input ! (input as LitStr) ; let span = branch_name . span () ; quote_spanned ! { span => eprintln ! ("\n🌿 BRANCH! Created new branch: `{}`\n" , # branch_name) ; () } . into () }
};
}
