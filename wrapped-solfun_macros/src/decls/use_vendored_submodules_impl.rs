macro_rules! use_vendored_submodules_impl {
    () => {
        # [decl (fn , name = "use_vendored_submodules_impl" , vis = "pub" , hash = "f667e8d2")] pub fn use_vendored_submodules_impl (_input : TokenStream) -> TokenStream { quote ! { eprintln ! ("\nVendored submodules configuration conceptually applied! 📦🔒\n") ; () } . into () }
    };
}

use_vendored_submodules_impl!();