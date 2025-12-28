macro_rules! deps {
    () => {
        Init!();
        InitLocation!();
    };
}

macro_rules! impl_203 {
    () => {
        deps!();
        impl Init { pub fn span < 'tcx > (& self , body : & Body < 'tcx >) -> Span { match self . location { InitLocation :: Argument (local) => body . local_decls [local] . source_info . span , InitLocation :: Statement (location) => body . source_info (location) . span , } } }
    };
}

impl_203!();