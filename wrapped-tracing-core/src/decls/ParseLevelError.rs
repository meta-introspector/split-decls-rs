macro_rules! deps {
    () => {
        Level!();
    };
}

macro_rules! ParseLevelError {
    () => {
        deps!();
        # [doc = " Returned if parsing a `Level` fails."] # [derive (Debug)] pub struct ParseLevelError { _p : () , }
    };
}

ParseLevelError!();