macro_rules! use_vendored_submodules {
    () => {
        # [proc_macro] # [decl (fn , name = "use_vendored_submodules" , vis = "pub" , hash = "927fa3d4")] pub fn use_vendored_submodules (input : TokenStream) -> TokenStream { macros :: use_vendored_submodules :: use_vendored_submodules_impl (input) }
    };
}

use_vendored_submodules!()