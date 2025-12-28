macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The Error type"] # [derive (Debug)] pub struct Error { # [doc = " Kind of error"] pub kind : ErrorKind , source : Option < Box < dyn StdError + Sync + Send > > , }
    };
}

Error!()