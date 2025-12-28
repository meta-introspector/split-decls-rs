macro_rules! deps {
    () => {
        Expr!();
        Fragment!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl AsRef < TokenStream > for Fragment { fn as_ref (& self) -> & TokenStream { match self { Fragment :: Expr (expr) => expr , Fragment :: Block (block) => block , } } }
    };
}

impl_173!()