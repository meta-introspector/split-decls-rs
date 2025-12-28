macro_rules! deps {
    () => {
        Exfiltrator!();
        PendingSignals!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < E : Exfiltrator > Debug for PendingSignals < E > { fn fmt (& self , fmt : & mut Formatter) -> FmtResult { fmt . debug_struct ("PendingSignals") . field ("exfiltrator" , & self . exfiltrator) . field ("slots" , & & self . slots [..]) . finish () } }
    };
}

impl_15!();