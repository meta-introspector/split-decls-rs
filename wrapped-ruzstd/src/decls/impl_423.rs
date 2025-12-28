macro_rules! deps {
    () => {
        Error!();
        ErrorKind!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl Error { pub fn new (kind : ErrorKind , err : Box < dyn core :: fmt :: Display + Send + Sync + 'static >) -> Self { Self { kind , err : Some (err) , } } pub fn from (kind : ErrorKind) -> Self { Self { kind , err : None } } pub fn kind (& self) -> ErrorKind { self . kind } pub fn is_interrupted (& self) -> bool { matches ! (self . kind , ErrorKind :: Interrupted) } pub fn get_ref (& self) -> Option < & (dyn core :: fmt :: Display + Send + Sync) > { self . err . as_ref () . map (| e | e . as_ref ()) } pub fn into_inner (self) -> Option < Box < dyn core :: fmt :: Display + Send + Sync + 'static > > { self . err } }
    };
}

impl_423!()