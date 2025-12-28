macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        pub struct Error { kind : ErrorKind , err : Option < Box < dyn core :: fmt :: Display + Send + Sync + 'static > > , }
    };
}

Error!();