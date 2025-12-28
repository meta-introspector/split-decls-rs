macro_rules! Error {
    () => {
        # [doc = "\nAn error encountered while streaming a value.\n\nErrors don't capture details of failures, that responsibility is left\nto the stream to surface.\n"] # [derive (Debug)] pub struct Error (()) ;
    };
}

Error!();