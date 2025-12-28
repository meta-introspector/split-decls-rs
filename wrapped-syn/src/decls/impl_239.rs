macro_rules! impl_239 {
    () => {
        impl From < Ident > for Member { fn from (ident : Ident) -> Member { Member :: Named (ident) } }
    };
}

impl_239!();