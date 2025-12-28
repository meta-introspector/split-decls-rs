macro_rules! default_db_lifetime {
    () => {
        # [doc = " Normally we try to use whatever lifetime parameter the user gave us"] # [doc = " to represent `'db`; but if they didn't give us one, we need to use a default"] # [doc = " name. We choose `'db`."] pub (crate) fn default_db_lifetime (span : Span) -> syn :: Lifetime { syn :: Lifetime { apostrophe : span , ident : syn :: Ident :: new ("db" , span) , } }
    };
}

default_db_lifetime!();