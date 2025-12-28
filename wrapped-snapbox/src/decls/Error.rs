macro_rules! deps {
    () => {
        Backtrace!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [derive (Clone , Debug)] pub struct Error { inner : String , backtrace : Option < Backtrace > , }
    };
}

Error!();