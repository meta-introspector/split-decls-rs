macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        # [derive (Debug)] pub struct Outcome < O , E > { # [doc = " Backtrace of obligations that were found to be in error."] pub errors : Vec < Error < O , E > > , }
    };
}

Outcome!()