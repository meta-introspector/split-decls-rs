macro_rules! deps {
    () => {
        IterImpl!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl IterImpl { pub fn peek (& mut self) -> Option < & TokenTree > { self . peeked = self . next () ; self . peeked . as_ref () } }
    };
}

impl_43!();