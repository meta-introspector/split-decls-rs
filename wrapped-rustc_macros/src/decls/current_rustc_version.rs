macro_rules! current_rustc_version {
    () => {
        # [proc_macro] pub fn current_rustc_version (input : TokenStream) -> TokenStream { current_version :: current_version (input) }
    };
}

current_rustc_version!();