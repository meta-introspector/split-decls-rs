macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! error {
    () => {
        deps!();
        # [doc = "\nA streaming result with a generic failure.\n\nMore detailed diagnostic information will need to be stored elsewhere.\n"] # [inline (always)] pub fn error < T > () -> crate :: Result < T > { Err (Error :: new ()) }
    };
}

error!()