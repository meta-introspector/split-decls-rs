macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! ThreadPoolBuildError {
    () => {
        deps!();
        # [doc = " Error when initializing a thread pool."] # [derive (Debug)] pub struct ThreadPoolBuildError { kind : ErrorKind , }
    };
}

ThreadPoolBuildError!()