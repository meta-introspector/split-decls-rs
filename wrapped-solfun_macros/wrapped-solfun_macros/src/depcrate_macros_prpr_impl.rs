// Generated macro for pr_impl (function)
macro_rules! Depcrate_macros_prpr_impl {
() => {
// Module: crate::macros::pr
// Provides: {"pr_impl"}
// Dependencies: {}
# [decl (fn , name = "pr_impl" , vis = "pub" , hash = "077c7684")] pub fn pr_impl (input : TokenStream) -> TokenStream { let description = parse_macro_input ! (input as LitStr) ; let span = description . span () ; quote_spanned ! { span => eprintln ! ("\n🤝 PULL REQUEST! Created PR: \"{{}}\"\n" , # description) ; () } . into () }
};
}
