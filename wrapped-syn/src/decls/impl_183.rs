macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl Clone for Error { fn clone (& self) -> Self { Error { messages : self . messages . clone () , } } }
    };
}

impl_183!();