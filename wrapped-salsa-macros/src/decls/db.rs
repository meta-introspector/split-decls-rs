macro_rules! db {
    () => {
        # [proc_macro_attribute] pub fn db (args : TokenStream , input : TokenStream) -> TokenStream { db :: db (args , input) }
    };
}

db!();