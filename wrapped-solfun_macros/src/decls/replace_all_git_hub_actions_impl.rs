macro_rules! replace_all_git_hub_actions_impl {
    () => {
        # [decl (fn , name = "replace_all_git_hub_actions_impl" , vis = "pub" , hash = "613ea2f0")] pub fn replace_all_git_hub_actions_impl (input : TokenStream) -> TokenStream { let new_workflow = parse_macro_input ! (input as LitStr) ; let span = new_workflow . span () ; quote_spanned ! { span => eprintln ! ("\n♻️ REPLACE ALL GITHUB ACTIONS! Conceptually replacing with: {}\n" , # new_workflow) ; () } . into () }
    };
}

replace_all_git_hub_actions_impl!();