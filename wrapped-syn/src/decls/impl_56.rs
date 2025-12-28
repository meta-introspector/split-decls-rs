macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Group { # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] pub fn surround < F > (& self , tokens : & mut TokenStream , f : F) where F : FnOnce (& mut TokenStream) , { let mut inner = TokenStream :: new () ; f (& mut inner) ; printing :: delim (Delimiter :: None , self . span , tokens , inner) ; } }
    };
}

impl_56!();