macro_rules! deps {
    () => {
        Punctuated!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl Default for Generics { fn default () -> Self { Generics { lt_token : None , params : Punctuated :: new () , gt_token : None , where_clause : None , } } }
    };
}

impl_295!()