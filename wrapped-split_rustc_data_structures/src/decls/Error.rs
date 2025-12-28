macro_rules! Error {
    () => {
        # [derive (Debug , PartialEq , Eq)] pub struct Error < O , E > { pub error : E , pub backtrace : Vec < O > , }
    };
}

Error!();