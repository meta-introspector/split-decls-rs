macro_rules! Error {
    () => {
        pub trait Error { fn new (msg : String) -> Self ; fn from_internal < T : Debug > (err : T) -> Self ; }
    };
}

Error!()