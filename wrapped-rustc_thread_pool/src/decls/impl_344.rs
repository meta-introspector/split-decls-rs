macro_rules! deps {
    () => {
        ErrorKind!();
        ThreadPoolBuildError!();
    };
}

macro_rules! impl_344 {
    () => {
        deps!();
        impl fmt :: Display for ThreadPoolBuildError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match & self . kind { ErrorKind :: GlobalPoolAlreadyInitialized => { "The global thread pool has already been initialized." . fmt (f) } ErrorKind :: IOError (e) => e . fmt (f) , } } }
    };
}

impl_344!()