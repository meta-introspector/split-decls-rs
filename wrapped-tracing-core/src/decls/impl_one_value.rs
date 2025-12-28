macro_rules! deps {
    () => {
        Field!();
        Value!();
        Visit!();
    };
}

macro_rules! impl_one_value {
    () => {
        deps!();
        macro_rules ! impl_one_value { (f32 , $ op : expr , $ record : ident) => { impl_one_value ! (normal , f32 , $ op , $ record) ; } ; (f64 , $ op : expr , $ record : ident) => { impl_one_value ! (normal , f64 , $ op , $ record) ; } ; (bool , $ op : expr , $ record : ident) => { impl_one_value ! (normal , bool , $ op , $ record) ; } ; ($ value_ty : tt , $ op : expr , $ record : ident) => { impl_one_value ! (normal , $ value_ty , $ op , $ record) ; impl_one_value ! (nonzero , $ value_ty , $ op , $ record) ; } ; (normal , $ value_ty : tt , $ op : expr , $ record : ident) => { impl $ crate :: sealed :: Sealed for $ value_ty { } impl $ crate :: field :: Value for $ value_ty { fn record (& self , key : &$ crate :: field :: Field , visitor : & mut dyn $ crate :: field :: Visit) { # [allow (clippy :: redundant_closure_call)] visitor .$ record (key , $ op (* self)) } } } ; (nonzero , $ value_ty : tt , $ op : expr , $ record : ident) => { # [allow (clippy :: useless_attribute , unused)] use num ::*; impl $ crate :: sealed :: Sealed for ty_to_nonzero ! ($ value_ty) { } impl $ crate :: field :: Value for ty_to_nonzero ! ($ value_ty) { fn record (& self , key : &$ crate :: field :: Field , visitor : & mut dyn $ crate :: field :: Visit) { # [allow (clippy :: redundant_closure_call)] visitor .$ record (key , $ op (self . get ())) } } } ; }
    };
}

impl_one_value!();