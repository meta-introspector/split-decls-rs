macro_rules! deps {
    () => {
        PollSendError!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < T > fmt :: Display for PollSendError < T > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (fmt , "channel closed") } }
    };
}

impl_57!();