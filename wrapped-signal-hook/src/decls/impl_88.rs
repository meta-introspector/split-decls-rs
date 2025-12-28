macro_rules! deps {
    () => {
        Origin!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl Debug for Origin { fn fmt (& self , fmt : & mut Formatter) -> FmtResult { fn named_signal (sig : c_int) -> String { low_level :: signal_name (sig) . map (| n | format ! ("{} ({})" , n , sig)) . unwrap_or_else (| | sig . to_string ()) } fmt . debug_struct ("Origin") . field ("signal" , & named_signal (self . signal)) . field ("process" , & self . process) . field ("cause" , & self . cause) . finish () } }
    };
}

impl_88!();