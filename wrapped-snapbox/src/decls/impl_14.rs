macro_rules! deps {
    () => {
        Error!();
        Backtrace!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Error { pub fn new (inner : impl std :: fmt :: Display) -> Self { Self :: with_string (inner . to_string ()) } fn with_string (inner : String) -> Self { Self { inner , backtrace : Backtrace :: new () , } } # [track_caller] pub (crate) fn panic (self) -> ! { panic ! ("{self}") } }
    };
}

impl_14!();