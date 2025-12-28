macro_rules! deps {
    () => {
        ErrorMessage!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl Clone for ErrorMessage { fn clone (& self) -> Self { ErrorMessage { span : self . span , message : self . message . clone () , } } }
    };
}

impl_184!();