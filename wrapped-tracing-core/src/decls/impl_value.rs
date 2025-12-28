macro_rules! impl_value {
    () => {
        macro_rules ! impl_value { ($ record : ident ($ ($ value_ty : tt) ,+)) => { $ (impl_one_value ! ($ value_ty , | this : $ value_ty | this , $ record) ;) + } ; ($ record : ident ($ ($ value_ty : tt) ,+ as $ as_ty : ty)) => { $ (impl_one_value ! ($ value_ty , | this : $ value_ty | this as $ as_ty , $ record) ;) + } ; }
    };
}

impl_value!();