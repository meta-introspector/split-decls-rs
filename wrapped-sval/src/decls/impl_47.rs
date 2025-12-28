macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Error { # [doc = "\n    Create a new error.\n\n    More detailed diagnostic information will need to be stored elsewhere.\n    "] # [inline (always)] pub fn new () -> Self { Error (()) } }
    };
}

impl_47!()