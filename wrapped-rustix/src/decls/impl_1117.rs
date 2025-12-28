macro_rules! deps {
    () => {
        Result!();
        WaitStatus!();
    };
}

macro_rules! impl_1117 {
    () => {
        deps!();
        impl fmt :: Debug for WaitStatus { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("WaitStatus") ; s . field ("stopped" , & self . stopped ()) ; s . field ("exited" , & self . exited ()) ; s . field ("signaled" , & self . signaled ()) ; s . field ("continued" , & self . continued ()) ; if let Some (stopping_signal) = self . stopping_signal () { s . field ("stopping_signal" , & stopping_signal) ; } if let Some (exit_status) = self . exit_status () { s . field ("exit_status" , & exit_status) ; } if let Some (terminating_signal) = self . terminating_signal () { s . field ("terminating_signal" , & terminating_signal) ; } s . finish () } }
    };
}

impl_1117!();