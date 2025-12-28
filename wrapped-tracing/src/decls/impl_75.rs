macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl Clone for Inner { fn clone (& self) -> Self { Inner { id : self . subscriber . clone_span (& self . id) , subscriber : self . subscriber . clone () , } } }
    };
}

impl_75!();