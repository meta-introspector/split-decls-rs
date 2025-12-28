macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! buffer_to_string {
    () => {
        deps!();
        pub (crate) fn buffer_to_string < F > (context : F , buffer : Vec < u8 >) -> Result < String , Error > where F : FnOnce () -> String , { String :: from_utf8 (buffer) . map_err (| error | Error :: utf8_conversion_error (error , context ())) }
    };
}

buffer_to_string!();