macro_rules! deps {
    () => {
        ErrorKind!();
        ThreadPoolBuildError!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Error for ThreadPoolBuildError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . kind { ErrorKind :: GlobalPoolAlreadyInitialized => None , ErrorKind :: IOError (e) => Some (e) , } } }
    };
}

impl_34!()