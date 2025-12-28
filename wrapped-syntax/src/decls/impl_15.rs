macro_rules! deps {
    () => {
        Parse!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < T > Clone for Parse < T > { fn clone (& self) -> Parse < T > { Parse { green : self . green . clone () , errors : self . errors . clone () , _ty : PhantomData } } }
    };
}

impl_15!()