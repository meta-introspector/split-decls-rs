macro_rules! deps {
    () => {
        ThreadPoolBuildError!();
        ErrorKind!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl Error for ThreadPoolBuildError { fn source (& self) -> Option < & (dyn Error + 'static) > { match & self . kind { ErrorKind :: GlobalPoolAlreadyInitialized => None , ErrorKind :: IOError (e) => Some (e) , } } }
    };
}

impl_343!();