macro_rules! impl_values {
    () => {
        macro_rules ! impl_values { ($ ($ record : ident ($ ($ whatever : tt) +)) ,+) => { $ (impl_value ! { $ record ($ ($ whatever) +) }) + } }
    };
}

impl_values!()